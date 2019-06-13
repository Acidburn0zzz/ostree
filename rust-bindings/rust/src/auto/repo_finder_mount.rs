// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use RepoFinder;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
use gio;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
use gobject_sys;
use ostree_sys;
use std::fmt;

glib_wrapper! {
    pub struct RepoFinderMount(Object<ostree_sys::OstreeRepoFinderMount, ostree_sys::OstreeRepoFinderMountClass, RepoFinderMountClass>) @implements RepoFinder;

    match fn {
        get_type => || ostree_sys::ostree_repo_finder_mount_get_type(),
    }
}

impl RepoFinderMount {
    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    pub fn new<P: IsA<gio::VolumeMonitor>>(monitor: Option<&P>) -> RepoFinderMount {
        unsafe {
            from_glib_full(ostree_sys::ostree_repo_finder_mount_new(monitor.map(|p| p.as_ref()).to_glib_none().0))
        }
    }
}

pub const NONE_REPO_FINDER_MOUNT: Option<&RepoFinderMount> = None;

pub trait RepoFinderMountExt: 'static {
    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    fn get_property_monitor(&self) -> Option<gio::VolumeMonitor>;
}

impl<O: IsA<RepoFinderMount>> RepoFinderMountExt for O {
    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    fn get_property_monitor(&self) -> Option<gio::VolumeMonitor> {
        unsafe {
            let mut value = Value::from_type(<gio::VolumeMonitor as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"monitor\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }
}

impl fmt::Display for RepoFinderMount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RepoFinderMount")
    }
}
