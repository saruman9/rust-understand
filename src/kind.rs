extern crate understand_sys;
extern crate log;
extern crate time;

use std::ffi::{CStr, CString};
use std::mem;
use std::fmt;

use understand_sys::{UdbKind, UdbKindList, udbKindLongname, udbKindShortname, udbIsKindFile,
udbKindLanguage, udbIsKind, udbKindInverse, udbKindList, udbListKindEntity, udbListKindFree,
udbListKindReference};

use language::Language;

/// Structure of Kind.
#[derive(Debug)]
pub struct Kind {
    raw: i32,
}

impl Kind {

    pub unsafe fn from_raw(raw: UdbKind) -> Self {
        Kind { raw: raw as i32 }
    }

    unsafe fn from_list_raw(raw: UdbKindList) -> Vec<Self> {
        unimplemented!();
    }

    /// Return the long name of kind as String.
    pub fn name_long(&self) -> String {
        unsafe {
            CStr::from_ptr(udbKindLongname(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the short name of kind as String.
    pub fn name_short(&self) -> String {
        unsafe{
            CStr::from_ptr(udbKindShortname(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the language of the kind.
    pub fn language(&self) -> Option<Language> {
        unsafe{
            Language::from_raw(udbKindLanguage(self.raw))
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
            Kind::from_raw(udbKindInverse(self.raw))
        }
    }

    /// Return Vec of all entity kinds.
    pub fn kinds_of_ents() -> Vec<Self> {
        unsafe {
            let mut list_kind_raw: *mut UdbKind = mem::uninitialized();
            let mut count_kinds: i32 = mem::uninitialized();

            let mut list_kind: Vec<Kind> = vec!();

            udbListKindEntity(&mut list_kind_raw, &mut count_kinds);
            for i in 0..count_kinds {
                list_kind.push(Kind::from_raw(*list_kind_raw.offset(i as isize)));
            }
            udbListKindFree(list_kind_raw);

            list_kind
        }
    }

    /// Return Vec of all reference kinds.
    pub fn kinds_of_refs() -> Vec<Self> {
        unsafe {
            let mut list_kinds_raw: *mut UdbKind = mem::uninitialized();
            let mut count_kinds: i32 = mem::uninitialized();

            let mut list_kind: Vec<Kind> = vec!();

            udbListKindReference(&mut list_kinds_raw, &mut count_kinds);
            for i in 0..count_kinds {
                list_kind.push(Kind::from_raw(*list_kinds_raw.offset(i as isize)));
            }
            udbListKindFree(list_kinds_raw);

            list_kind
        }
    }

}
