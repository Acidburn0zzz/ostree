extern crate gio;
extern crate glib;
extern crate openat;
extern crate ostree;
extern crate tempfile;
#[macro_use]
extern crate maplit;

mod util;
use util::*;

use gio::prelude::*;
use gio::NONE_CANCELLABLE;
use glib::prelude::*;
use ostree::{
    ObjectName, ObjectType, RepoCheckoutAtOptions, RepoCheckoutFilterResult, RepoCheckoutMode,
    RepoCheckoutOverwriteMode, RepoDevInoCache,
};
use std::os::unix::io::AsRawFd;

#[test]
fn should_commit_content_to_repo_and_list_refs_again() {
    let test_repo = TestRepo::new();

    let mtree = create_mtree(&test_repo.repo);
    let checksum = commit(&test_repo.repo, &mtree, "test");

    let repo = ostree::Repo::new_for_path(test_repo.dir.path());
    repo.open(NONE_CANCELLABLE).expect("OSTree test_repo");
    let refs = repo
        .list_refs(None, NONE_CANCELLABLE)
        .expect("failed to list refs");
    assert_eq!(1, refs.len());
    assert_eq!(checksum, refs["test"]);
}

#[test]
fn should_traverse_commit() {
    let test_repo = TestRepo::new();
    let checksum = test_repo.test_commit("test");

    let objects = test_repo
        .repo
        .traverse_commit(&checksum, -1, NONE_CANCELLABLE)
        .expect("traverse commit");

    assert_eq!(
        hashset!(
            ObjectName::new(
                "89f84ca9854a80e85b583e46a115ba4985254437027bad34f0b113219323d3f8",
                ObjectType::File
            ),
            ObjectName::new(
                "5280a884f930cae329e2e39d52f2c8e910c2ef4733216b67679db32a2b56c4db",
                ObjectType::DirTree
            ),
            ObjectName::new(
                "c81acde323d73f8639fc84f1ded17bbafc415e645f845e9f3b16a4906857c2d4",
                ObjectType::DirTree
            ),
            ObjectName::new(
                "ad49a0f4e3bc165361b6d17e8a865d479b373ee67d89ac6f0ce871f27da1be6d",
                ObjectType::DirMeta
            ),
            ObjectName::new(checksum, ObjectType::Commit)
        ),
        objects
    );
}

#[test]
fn should_checkout_tree() {
    let test_repo = TestRepo::new();
    let _ = test_repo.test_commit("test");

    let checkout_dir = tempfile::tempdir().expect("checkout dir");
    let file = test_repo
        .repo
        .read_commit("test", NONE_CANCELLABLE)
        .expect("read commit")
        .0
        .downcast::<ostree::RepoFile>()
        .expect("RepoFile");
    let info = file
        .query_info("*", gio::FileQueryInfoFlags::NONE, NONE_CANCELLABLE)
        .expect("file info");
    test_repo
        .repo
        .checkout_tree(
            ostree::RepoCheckoutMode::User,
            ostree::RepoCheckoutOverwriteMode::None,
            &gio::File::new_for_path(checkout_dir.path().join("test-checkout")),
            &file,
            &info,
            NONE_CANCELLABLE,
        )
        .expect("checkout tree");

    assert_test_file(checkout_dir.path());
}

#[test]
#[cfg(feature = "v2016_8")]
fn should_checkout_at_with_none_options() {
    let test_repo = TestRepo::new();
    let checksum = test_repo.test_commit("test");
    let checkout_dir = tempfile::tempdir().expect("checkout dir");

    let dirfd = openat::Dir::open(checkout_dir.path()).expect("openat");
    test_repo
        .repo
        .checkout_at(
            None,
            dirfd.as_raw_fd(),
            "test-checkout",
            &checksum,
            NONE_CANCELLABLE,
        )
        .expect("checkout at");

    assert_test_file(checkout_dir.path());
}

#[test]
#[cfg(feature = "v2016_8")]
fn should_checkout_at_with_default_options() {
    let test_repo = TestRepo::new();
    let checksum = test_repo.test_commit("test");
    let checkout_dir = tempfile::tempdir().expect("checkout dir");

    let dirfd = openat::Dir::open(checkout_dir.path()).expect("openat");
    test_repo
        .repo
        .checkout_at(
            Some(&RepoCheckoutAtOptions::default()),
            dirfd.as_raw_fd(),
            "test-checkout",
            &checksum,
            NONE_CANCELLABLE,
        )
        .expect("checkout at");

    assert_test_file(checkout_dir.path());
}

#[test]
#[cfg(feature = "v2016_8")]
fn should_checkout_at_with_options() {
    let test_repo = TestRepo::new();
    let checksum = test_repo.test_commit("test");
    let checkout_dir = tempfile::tempdir().expect("checkout dir");

    let dirfd = openat::Dir::open(checkout_dir.path()).expect("openat");
    test_repo
        .repo
        .checkout_at(
            Some(&RepoCheckoutAtOptions {
                mode: RepoCheckoutMode::User,
                overwrite_mode: RepoCheckoutOverwriteMode::AddFiles,
                enable_fsync: true,
                force_copy: true,
                force_copy_zerosized: true,
                devino_to_csum_cache: Some(RepoDevInoCache::new()),
                filter: Some(Box::new(|_repo, _path, _stat| {
                    RepoCheckoutFilterResult::Allow
                })),
                ..Default::default()
            }),
            dirfd.as_raw_fd(),
            "test-checkout",
            &checksum,
            NONE_CANCELLABLE,
        )
        .expect("checkout at");

    assert_test_file(checkout_dir.path());
}

#[test]
#[cfg(feature = "v2016_8")]
fn should_checkout_at_with_filter() {
    let test_repo = TestRepo::new();
    let checksum = test_repo.test_commit("test");
    let checkout_dir = tempfile::tempdir().expect("checkout dir");

    let dirfd = openat::Dir::open(checkout_dir.path()).expect("openat");
    test_repo
        .repo
        .checkout_at(
            Some(&RepoCheckoutAtOptions {
                filter: Some(Box::new(|_repo, path, _stat| {
                    if let Some("testfile") = path.file_name().map(|s| s.to_str().unwrap()) {
                        RepoCheckoutFilterResult::Skip
                    } else {
                        RepoCheckoutFilterResult::Allow
                    }
                })),
                ..Default::default()
            }),
            dirfd.as_raw_fd(),
            "test-checkout",
            &checksum,
            NONE_CANCELLABLE,
        )
        .expect("checkout at");

    let testdir = checkout_dir.path().join("test-checkout").join("testdir");
    assert!(std::fs::read_dir(&testdir).is_ok());
    assert!(std::fs::File::open(&testdir.join("testfile")).is_err());
}
