extern crate understand_sys;

use std::ffi::{CStr, CString};

use language::Language;

use understand_sys::{UdbKind, udbKindLongname, udbKindShortname, udbIsKindFile,
udbKindLanguage, udbIsKind, udbKindInverse};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Kind {
    pub raw: UdbKind,
}

impl Kind {
    pub fn from_raw_kind(kind: UdbKind) -> Self {
        Kind {raw: kind }
    }
    /// Return the long name of kind as String.
    pub fn get_name_long(&self) -> String {
        unsafe{
            CStr::from_ptr(udbKindLongname(self.raw)).to_string_lossy().into_owned()
        }
    }
    /// Return the short name of kind as String.
    pub fn get_name_short(&self) -> String {
        unsafe{
            CStr::from_ptr(udbKindShortname(self.raw)).to_string_lossy().into_owned()
        }
    }
    /// Return the language of the kind.
    pub fn get_language(&self) -> Option<Language> {
        unsafe{
            Language::from_raw_language(udbKindLanguage(self.raw))
        }
    }
    /// Return true if the kind refers to a file entity.
    pub fn is_file(&self) -> bool {
        unsafe{
            match udbIsKindFile(self.raw) {
                0 => false,
                _ => true,
            }
        }
    }
    /// Return true if the kind matches the kind text.
    pub fn is_kind(&self, text: &str) -> bool {
        unsafe {
            let text: CString = CString::new(text).unwrap();
            match udbIsKind(self.raw, text.as_ptr()) {
                0 => false,
                _ => true,
            }
        }
    }
    /// Return the inverse of the reference kind.
    pub fn inverse(&self) -> Self {
        unsafe {
            Kind::from_raw_kind(udbKindInverse(self.raw))
        }
    }
}

/*
    // Add a kind to the kindlist if not 0 or allocate a new kindlist.
    pub fn udbKindList(kind     : UdbKind,
                       kindlist : *mut UdbKindList);

    // Return an allocated copy of kindlist that must be freed with
    // udbKindListFree()
    pub fn udbKindListCopy(kindlist: UdbKindList) -> UdbKindList;

    // Free an allocated kindlist.
    pub fn udbKindListFree(kindlist: UdbKindList);

    // Return true if kind is in the kindlist.
    pub fn udbKindLocate(kind     : UdbKind,
                         kindlist : UdbKindList) -> c_int;

    // Parse the kind text and return an allocated kindlist that must be freed
    // with udbKindListFree().
    pub fn udbKindParse(text: *const c_char) -> UdbKindList;


    // Return allocated list of all entity kinds. Call udbListKindFree() to free
    // list.
    pub fn udbListKindEntity(list  : *mut *mut UdbKind,
                             items : *mut c_int);

    // Free an allocated list of kinds.
    pub fn udbListKindFree(list: *mut UdbKind);

    // Return allocated list of all reference kinds. Call udbListKindFree() to
    // free list.
    pub fn udbListKindReference(list  : *mut *mut UdbKind,
                                items : *mut c_int);
*/
