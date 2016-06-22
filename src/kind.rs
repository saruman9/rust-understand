extern crate understand_sys;

use std::ffi::CStr;

use language::Language;

use understand_sys::{UdbKind, udbKindLongname, udbKindShortname, udbIsKindFile,
udbKindLanguage};

#[derive(Clone)]
pub struct Kind {
    pub name_long: String,
    pub name_short: String,
    pub is_file: bool,
    pub language: Option<Language>,
}

impl Kind {
    pub fn from_raw_kind(kind: UdbKind) -> Self {
        unsafe {
            let name_long: String = CStr::from_ptr(udbKindLongname(kind))
                .to_string_lossy().into_owned();
            let name_short: String = CStr::from_ptr(udbKindShortname(kind))
                .to_string_lossy().into_owned();
            let is_file: bool;
            if udbIsKindFile(kind) != 0 {
                is_file = true;
            } else {
                is_file = false;
            }
            let language: Option<Language> = Language::from_raw_language(udbKindLanguage(kind));

            Kind {
                name_long: name_long,
                name_short: name_short,
                is_file: is_file,
                language: language,
            }
        }
    }
}

/*
    // Return true if the kind matches the kind text.
    pub fn udbIsKind(kind: UdbKind,
                     text: *const c_char) -> c_int;

    // Return true if the kind refers to a file entity.
    pub fn udbIsKindFile(kind: UdbKind) -> c_int;

    // Return the inverse of the reference kind.
    pub fn udbKindInverse(kind: UdbKind) -> UdbKind;

    // Add a kind to the kindlist if not 0 or allocate a new kindlist.
    pub fn udbKindList(kind     : UdbKind,
                       kindlist : *mut UdbKindList);

    // Return the language of the kind.
    pub fn udbKindLanguage(kind: UdbKind) -> UdbLanguage;

    // Return an allocated copy of kindlist that must be freed with
    // udbKindListFree()
    pub fn udbKindListCopy(kindlist: UdbKindList) -> UdbKindList;

    // Free an allocated kindlist.
    pub fn udbKindListFree(kindlist: UdbKindList);

    // Return true if kind is in the kindlist.
    pub fn udbKindLocate(kind     : UdbKind,
                         kindlist : UdbKindList) -> c_int;

    // Return the long name of kind as a temporary string.
    pub fn udbKindLongname(kind: UdbKind) -> *const c_char;

    // Parse the kind text and return an allocated kindlist that must be freed
    // with udbKindListFree().
    pub fn udbKindParse(text: *const c_char) -> UdbKindList;

    // Return the short name of kind as a temporary string.
    pub fn udbKindShortname(kind: UdbKind) -> *const c_char;

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
