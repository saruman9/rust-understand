extern crate understand_sys;

use std::mem;
use std::ffi::CStr;

use language::Language;
use kind::Kind;

use understand_sys::{UdbReference, UdbKind, UdbLibrary, UdbEntity, udbEntityId,
udbEntityNameUnique, udbEntityNameLong, udbEntityNameSimple,
udbEntityNameAbsolute, udbEntityNameShort, udbEntityNameRelative, udbEntityKind};


pub struct Entity<'ent> {
    pub id            : i32,
    pub name_unique   : &'ent str,
    pub name_long     : &'ent str,
    pub name_simple   : &'ent str,
    pub name_short    : &'ent str,
    pub kind          : Kind<'ent>,
    pub language      : Option<Language>,
    pub library       : Option<UdbLibrary>,
    pub contents      : Option<&'ent str>,
    pub references    : Option<Vec<UdbReference>>,
    pub typetext      : Option<&'ent str>,
    pub freetext      : Option<&'ent str>,
    pub parameters    : Option<Vec<&'ent str>>,
    pub value         : Option<&'ent str>,
    // TODO Remove?
    pub name_absolute : Option<&'ent str>,
    pub name_relative : Option<&'ent str>,
}

impl<'ent> Entity<'ent> {
    pub fn from_raw_list_ents(udb_list_ents: *mut UdbEntity, udb_count_ents: i32) -> Option<Vec<Self>> {
        let mut ret: Vec<Entity> = vec!();
        unsafe {
            for i in 0..udb_count_ents {
                let entity: UdbEntity = *udb_list_ents.offset(i as isize);
                let id: i32 = udbEntityId(entity) as i32;
                let name_unique: &str = CStr::from_ptr(
                    udbEntityNameUnique(entity))
                    .to_str().unwrap();
                let name_long: &str = CStr::from_ptr(
                    udbEntityNameLong(entity))
                    .to_str().unwrap();
                let name_simple: &str = CStr::from_ptr(
                    udbEntityNameSimple(entity))
                    .to_str().unwrap();
                let name_short: &str = CStr::from_ptr(
                    udbEntityNameShort(entity))
                    .to_str().unwrap();
                let kind: Kind = Kind::from_raw_kind(udbEntityKind(entity));
                /*
                let name_absolute: &CStr = CStr::from_ptr(
                    udbEntityNameAbsolute(entity));
                let name_relative: Option<&str> = CStr::from_ptr(
                    udbEntityNameRelative(entity))
                    .to_str().ok();
                */

                ret.push(Entity{
                    id            : id,
                    name_unique   : name_unique,
                    name_long     : name_long,
                    name_simple   : name_simple,
                    name_short    : name_short,
                    name_absolute : None,
                    name_relative : None,
                    kind          : kind,
                    language      : None,
                    library       : None,
                    contents      : None,
                    references    : None,
                    typetext      : None,
                    freetext      : None,
                    parameters    : None,
                    value         : None,
                });
            }
        }
        match ret.is_empty() {
            false => Some(ret),
            true  => None
        }
    }
}
