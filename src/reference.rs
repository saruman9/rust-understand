extern crate understand_sys;

use understand_sys::{UdbReference, udbReferenceLine, udbReferenceColumn,
udbReferenceEntity, udbEntityId};

#[derive(Clone)]
pub struct Reference {
    pub column: i32,
    pub line: i32,
    pub entity_id: i32,
}

impl Reference {
    pub fn from_raw_reference(reference: UdbReference) -> Self {
        unsafe {
            let line: i32 = udbReferenceLine(reference) as i32;
            let column: i32 = udbReferenceColumn(reference) as i32;
            let entity_id: i32 = udbEntityId(udbReferenceEntity(reference));
            Reference {
                column: column,
                line: line,
                entity_id: entity_id,
            }
        }
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

    // Return reference kind.
    pub fn udbReferenceKind(reference: UdbReference) -> UdbKind;

    // Return reference scope.
    pub fn udbReferenceScope(reference: UdbReference) -> UdbEntity;
*/
