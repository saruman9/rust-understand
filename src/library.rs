extern crate understand_sys;

use std::ffi::CStr;
use std::fmt;

use understand_sys::{UdbLibrary, udbLibraryName};

pub struct Library {
    raw: UdbLibrary,
}

impl Library {
    pub unsafe fn from_raw(library: UdbLibrary) -> Self {
        Library { raw: library }
    }

    /// Return the library name as a temporary string. Never return NULL.
    pub fn name(&self) -> Option<&str> {
        unsafe { CStr::from_ptr(udbLibraryName(self.raw)).to_str().ok() }
    }

    /// Return true if the library is "Standard" and false else.
    pub fn is_standard(&self) -> bool {
        self.name().unwrap_or_default() == "Standard"
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name().unwrap_or_default())
    }
}
