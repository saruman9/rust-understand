extern crate understand_sys;

use std::ffi::CStr;
use std::fmt;

use understand_sys::{UdbLibrary, udbLibraryName};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Library {
    pub name: String,
}

impl Library {
    pub fn from_raw_library(library: UdbLibrary) -> Option<Self> {
        unsafe {
            let name: String = CStr::from_ptr(udbLibraryName(library)).to_string_lossy().into_owned();
            if name.is_empty() {
                None
            } else {
                Some(
                    Library {
                        name: name,
                    }
                )
            }
        }
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
