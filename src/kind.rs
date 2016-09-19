extern crate understand_sys;
extern crate log;
extern crate time;

use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::fmt;

use understand_sys::{UdbKind, UdbKindList, udbKindLongname, udbKindShortname, udbIsKindFile,
                     udbKindLanguage, udbIsKind, udbKindInverse, udbListKindEntity,
                     udbListKindFree, udbListKindReference, udbKindParse, udbKindListFree};

use language::Language;

/// Structure of Kind.
#[derive(Debug)]
pub struct Kind {
    raw: i32,
}

pub trait KindVec {
    /// Return true if kind is in the kindlist.
    fn locate(&self, kind: &Kind) -> bool;
}

impl Kind {
    pub unsafe fn from_raw(raw: UdbKind) -> Self {
        Kind { raw: raw as i32 }
    }

    /// Scheme of UdbKindList:
    /// *const (length: i64, kinds: *const [i32; length]);
    /// length right shifted on 0x20 (length >> 0x20).
    /// TODO Expect error on 32bit machines.
    unsafe fn from_raw_list(raw: UdbKindList) -> Vec<Kind> {
        let mut kinds: Vec<Kind> = vec![];
        if raw.is_null() {
            return kinds;
        }
        let raw_ptr: *const i64 = mem::transmute(raw);
        let len: i64 = ptr::read(raw_ptr) >> 0x20;
        let raw_arr = ptr::read(raw_ptr.offset(1)) as *const i32;
        for i in 0..len {
            let raw_kind = ptr::read(raw_arr.offset(i as isize));
            kinds.push(Kind::from_raw(raw_kind));
        }
        udbKindListFree(raw);
        kinds
    }

    /// Parse the kind text.
    pub fn parse(text: &str) -> Vec<Kind> {
        unsafe { Kind::from_raw_list(udbKindParse(CString::new(text).unwrap().as_ptr())) }
    }

    /// Return the long name of kind as String.
    pub fn name_long(&self) -> String {
        unsafe { CStr::from_ptr(udbKindLongname(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return the short name of kind as String.
    pub fn name_short(&self) -> String {
        unsafe { CStr::from_ptr(udbKindShortname(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return the language of the kind.
    pub fn language(&self) -> Option<Language> {
        unsafe { Language::from_raw(udbKindLanguage(self.raw)) }
    }

    /// Return true if the kind refers to a file entity.
    pub fn is_file(&self) -> bool {
        unsafe {
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
    pub fn inverse(&self) -> Option<Self> {
        unsafe {
            let kind_inv: UdbKind = udbKindInverse(self.raw);
            if kind_inv == 0 {
                None
            } else {
                Some(Kind::from_raw(kind_inv))
            }
        }
    }

    /// Return Vec of all entity kinds.
    pub fn kinds_of_ents() -> Vec<Self> {
        unsafe {
            let mut list_kind_raw: *mut UdbKind = mem::uninitialized();
            let mut count_kinds: i32 = mem::uninitialized();

            let mut list_kind: Vec<Kind> = vec![];

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

            let mut list_kind: Vec<Kind> = vec![];

            udbListKindReference(&mut list_kinds_raw, &mut count_kinds);
            for i in 0..count_kinds {
                list_kind.push(Kind::from_raw(*list_kinds_raw.offset(i as isize)));
            }
            udbListKindFree(list_kinds_raw);

            list_kind
        }
    }
}

impl KindVec for Vec<Kind> {
    fn locate(&self, kind: &Kind) -> bool {
        for k in self {
            if k.raw == kind.raw {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name_long())
    }
}
