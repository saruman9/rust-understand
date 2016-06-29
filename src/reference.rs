extern crate understand_sys;

use std::fmt;

use understand_sys::{UdbReference, udbReferenceLine, udbReferenceColumn,
udbReferenceEntity, udbEntityId, udbReferenceKind};

use kind::Kind;
use entity::Entity;

#[derive(Clone)]
pub struct Reference {
    pub raw: UdbReference,
}

impl Reference {
    pub fn from_raw_reference(reference: UdbReference) -> Self {
        Reference { raw: reference }
    }
    pub fn from_raw_list_refs(udb_list_refs: *mut UdbReference, udb_count_refs: i32) -> Option<Vec<Self>> {
        let mut ret: Vec<Reference> = vec!();
        unsafe {
            for i in 0..udb_count_refs {
                let reference: UdbReference = *udb_list_refs.offset(i as isize);
                ret.push(Reference::from_raw_reference(reference));
            }
        }
        match ret.is_empty() {
            false => Some(ret),
            true  => None,
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
    //pub fn get_entity(&self) -> Enti
    //let entity_id: i32 = udbEntityId(udbReferenceEntity(reference));
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.get_kind().get_name_long(), self.get_line(), self.get_column())
    }
}

/*

    // Return an allocated copy of reference.
    pub fn udbReferenceCopy(reference: UdbReference) -> UdbReference;

    // Free reference copied by udbReferenceCopy().
    pub fn udbReferenceCopyFree(reference: UdbReference);

    // Return reference entity.
    pub fn udbReferenceEntity(reference: UdbReference) -> UdbEntity;

    // Return reference file.
    pub fn udbReferenceFile(reference: UdbReference) -> UdbEntity;

    // Return reference scope.
    pub fn udbReferenceScope(reference: UdbReference) -> UdbEntity;
*/
