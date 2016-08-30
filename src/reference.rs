extern crate understand_sys;
extern crate log;
extern crate time;

use std::fmt;
use std::marker::PhantomData;
use std::ops::Range;

use understand_sys::{UdbReference, UdbEntity, udbReferenceLine, udbReferenceColumn,
udbReferenceEntity, udbReferenceKind, udbReferenceScope, udbListReferenceFree, udbReferenceFile,
udbKindInverse};

use db::Db;
use entity::Entity;
use kind::Kind;

/// Structure of Reference.
pub struct Reference<'refs> {
    raw: UdbReference,
    _marker: PhantomData<&'refs UdbReference>,
}

/// Opaque structure of list of references.
pub struct ListReference<'db> {
    raw: *mut UdbReference,
    len: usize,
    _marker: PhantomData<&'db Db>,
}

/// An iterator over th Reference in list of references.
pub struct ReferenceIter<'refs> {
    range: Range<usize>,
    refs: &'refs ListReference<'refs>,
}

impl<'db> ListReference<'db> {

    pub unsafe fn from_raw(raw: *mut UdbReference, len: i32) -> ListReference<'db> {
        debug!("Created ListReference from {:?} with {} length at {}",
               raw,
               len,
               time::now().strftime("%M:%S.%f").unwrap());

        ListReference {
            raw: raw,
            len: len as usize,
            _marker: PhantomData,
        }
    }

    /// Gets the number of references that exist in the ListReference.
    pub fn len(&self) -> usize { self.len }

    ///Gets the Reference at the given index.
    pub fn get_index(&self, index: usize) -> Option<Reference> {
        unsafe {
            if index < self.len {
                Some(Reference::from_raw(*self.raw.offset(index as isize)))
            } else {
                None
            }
        }
    }

    pub fn iter(&self) -> ReferenceIter {
        ReferenceIter {
            range: 0..self.len,
            refs: self,
        }
    }

    /*
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
    */

}

impl<'refs> Reference<'refs> {

    unsafe fn from_raw(raw: UdbReference) -> Reference<'refs> {
        debug!("Created Reference from {:?} at {}",
               raw,
               time::now().strftime("%M:%S.%f").unwrap());

        Reference {
            raw: raw,
            _marker: PhantomData,
        }
    }

    /// Return reference line.
    pub fn line(&self) -> i32 {
        unsafe{ udbReferenceLine(self.raw) as i32 }
    }
    /// Return reference column.
    pub fn column(&self) -> i32 {
        unsafe{ udbReferenceColumn(self.raw) as i32 }
    }
    /// Return reference entity.
    pub fn entity(&self) -> Entity {
        unsafe { Entity::from_raw(udbReferenceEntity(self.raw)) }
    }
    /// Return reference scope.
    pub fn scope(&self) -> Entity {
        unsafe { Entity::from_raw(udbReferenceScope(self.raw)) }
    }
    /// Return reference file.
    pub fn file(&self) -> Entity {
        unsafe { Entity::from_raw(udbReferenceFile(self.raw)) }
    }

    /// Return reference kind.
    pub fn kind(&self) -> Kind {
        unsafe{ Kind::from_raw(udbReferenceKind(self.raw)) }
    }
}

impl<'refs> Iterator for ReferenceIter<'refs> {
    type Item = Reference<'refs>;

    fn next(&mut self) -> Option<Reference<'refs>> {
        self.range.next().and_then(|i| self.refs.get_index(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    fn count(self) -> usize {
        self.size_hint().0
    }
}

impl<'refs, 'iter> IntoIterator for &'iter ListReference<'refs> {
    type Item = Reference<'iter>;
    type IntoIter = ReferenceIter<'iter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'refs> DoubleEndedIterator for ReferenceIter<'refs> {

    fn next_back(&mut self) -> Option<Reference<'refs>> {
        self.range.next_back().and_then(|i| self.refs.get_index(i))
    }
}

impl<'db> fmt::Debug for ListReference<'db> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.raw, self.len)
    }
}

impl<'refs> fmt::Debug for Reference<'refs> {

    /// TODO write issue - don't expect lifetimes errors in std::format!()
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{scope} ({line}) {kind}
    kind->longname: {kind}
    ent->name: {ent}
    scope->name: {scope}
    file->longname: {file}
    line: {line}
    column: {column}",
                ent=self.entity().name_short(),
                scope=self.scope().name_short(),
                line=self.line(),
                column=self.column(),
                kind=self.kind().name_short(),
                file=self.file().name_long())
    }
}

impl<'db> Drop for ListReference<'db> {
    fn drop(&mut self) {
        debug!("Dropped ListReference {:?} at {}",
               self.raw,
               time::now().strftime("%M:%S.%f").unwrap());

        unsafe { udbListReferenceFree(self.raw) };
    }
}

/*
    // Return an allocated copy of reference.
    pub fn udbReferenceCopy(reference: UdbReference) -> UdbReference;

    // Free reference copied by udbReferenceCopy().
    pub fn udbReferenceCopyFree(reference: UdbReference);

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
*/
