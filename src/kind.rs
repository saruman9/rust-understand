extern crate understand_sys;
extern crate log;
extern crate time;

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Range;

use understand_sys::{UdbKind, UdbKindList, udbKindLongname, udbKindShortname, udbIsKindFile,
udbKindLanguage, udbIsKind, udbKindInverse, udbKindList};

use db::Db;
use language::Language;


/// Structure of Kind.
pub struct Kind<'kinds> {
    raw: UdbKind,
    _marker: PhantomData<&'kinds UdbKind>,
}

/// Opaque structure of list of kinds.
pub struct ListKind<'db> {
    raw: *mut UdbKind,
    len: usize,
    _marker: PhantomData<&'db Db>,
}

/// An iterator over the Kind in list of kinds.
pub struct KindIter<'kinds> {
    range: Range<usize>,
    kinds: &'kinds ListKind<'kinds>,
}

impl<'db> ListKind<'db> {

    pub unsafe fn from_raw(raw: *mut UdbKind, len: i32) -> ListKind<'db> {
        debug!("Created ListKind from {:?} with {} length at {}",
               raw,
               len,
               time::now().strftime("%M:%S.%f").unwrap());
        ListKind {
            raw: raw,
            len: len as usize,
            _marker: PhantomData,
        }
    }

    /// Gets the number of kinds that exist in the ListKind.
    pub fn len(&self) -> usize { self.len }

    /// Gets the Kind at the given index.
    pub fn get_index(&self, index: usize) -> Option<Kind> {
        unsafe {
            if index < self.len {
                Some(Kind::from_raw(*self.raw.offset(index as isize)))
            } else { None }
        }
    }

    /// Returns an iterator over the Kind in list of kinds.
    pub fn iter(&self) -> KindIter {
        KindIter {
            range: 0..self.len(),
            kinds: self,
        }
    }

    /* UdbKindList functions.

    // Lookup the comments associated with the specified entity and return a
    // temporary, formatted string.
    pub fn udbComment(entity : UdbEntity,
                      style  : UdbCommentStyle,
                      format : UdbCommentFormat,
                      kinds  : UdbKindList) -> *const c_char;

    // Lookup the comments associated with the specified entity and return a
    // temporary array of raw comment strings.
    pub fn udbCommentRaw(entity        : UdbEntity,
                         style         : UdbCommentStyle,
                         kinds         : UdbKindList,
                         commentString : *mut *const *const c_char,
                         len           : *mut c_int);

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

    // Filter the specified list of entities, using the kinds specified, and return
    // a new allocated array. Use udbListEntityFree() to free this list. The
    // original list of entities and the kindlist must both be allocated and will
    // be freed by this call.
    pub fn udbListEntityFilter(ents    : *mut UdbEntity,
                               kinds   : UdbKindList,
                               newents : *mut *mut UdbEntity,
                               items   : *mut c_int);

    // Filter the specified list of references, using the refkinds and/or the
    // entkinds specified, and return a new allocated array. If unique is
    // specified, the newrefs array will only contain the first reference for
    // each unique entity. Refkinds and Entkinds must both be allocated and
    // will be freed by this call.
    pub fn udbListReferenceFilter(refs     : *mut UdbReference,
                                  refkinds : UdbKindList,
                                  entkinds : UdbKindList,
                                  unique   : c_int,
                                  refs     : *mut *mut UdbReference,
                                  num      : *mut c_int);

    // Return true if the specified entity has any reference of the general kind
    // specified by the list of references. Return true if the list is 0. Kindlist
    // must be allocated and will be deleted.
    pub fn udbLookupReferenceExists(entity   : UdbEntity,
                                    kindlist : UdbKindList) -> c_int;

    */
}

impl<'kinds> Kind<'kinds> {

    pub unsafe fn from_raw(raw: UdbKind) -> Kind<'kinds> {
        debug!("Created Kind from {:?} at {}",
               raw,
               time::now().strftime("%M:%S.%f").unwrap());
        Kind {
            raw: raw,
            _marker: PhantomData,
        }
    }

    /// Return the long name of kind as String.
    pub fn name_long(&self) -> String {
        unsafe{
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
    pub fn inverse(&self) -> Kind<'kinds> {
        unsafe {
            Kind::from_raw(udbKindInverse(self.raw))
        }
    }
}

/*
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
