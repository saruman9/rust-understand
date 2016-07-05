extern crate understand_sys;

use std::fmt;

use understand_sys::{UdbReference, udbReferenceLine, udbReferenceColumn, udbReferenceEntity,
udbReferenceKind, udbReferenceScope, udbListReferenceFree, udbReferenceFile};

use kind::Kind;
use entity::Entity;

#[derive(Clone)]
pub struct Reference {
    pub raw: UdbReference,
}

pub struct ListReference {
    pub raw: *mut UdbReference,
    pub list: Vec<Reference>,
}

impl Reference {
    pub fn from_raw_reference(reference: UdbReference) -> Self {
        Reference { raw: reference }
    }
    pub fn from_raw_list_refs(udb_list_refs: *mut UdbReference, udb_count_refs: i32)
                              -> Option<ListReference> {
        let mut ret: Vec<Reference> = vec!();
        unsafe {
            for i in 0..udb_count_refs {
                let reference: UdbReference = *udb_list_refs.offset(i as isize);
                ret.push(Reference::from_raw_reference(reference));
            }
        }
        match ret.is_empty() {
            false => Some(ListReference {
                raw: udb_list_refs,
                list: ret,
            }),
            true => None,
        }
    }
    pub fn get_line(&self) -> i32 {
        unsafe{ udbReferenceLine(self.raw) as i32 }
    }
    pub fn get_column(&self) -> i32 {
        unsafe{ udbReferenceColumn(self.raw) as i32 }
    }
    pub fn get_kind(&self) -> Kind {
        unsafe{ Kind::from_raw_kind(udbReferenceKind(self.raw)) }
    }
    /// Return reference entity.
    pub fn get_entity(&self) -> Entity {
        unsafe { Entity::from_raw_entity(udbReferenceEntity(self.raw)) }
    }
    /// Return reference scope.
    pub fn get_scope(&self) -> Entity {
        unsafe { Entity::from_raw_entity(udbReferenceScope(self.raw)) }
    }
    /// Return reference file.
    pub fn get_file(&self) -> Entity {
        unsafe { Entity::from_raw_entity(udbReferenceFile(self.raw)) }
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{scope} ({line}) {kind}
    kind->longname: {kind}
    ent->name: {ent}
    scope->name: {scope}
    file->longname: {file}
    line: {line}
    column: {column}",
                ent=self.get_entity().get_name_short(),
                line=self.get_line(),
                column=self.get_column(),
                scope=self.get_scope().get_name_short(),
                kind=self.get_kind().get_name_short(),
                file=self.get_file().get_name_long())
    }
}

impl Drop for ListReference {
    fn drop(&mut self) {
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
