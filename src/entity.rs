extern crate understand_sys;

use std::mem;

use language::Language;

use understand_sys::{UdbReference, UdbKind, UdbLibrary, UdbEntity, udbEntityId};


pub struct Entity<'ent> {
    pub id            : i32,
    pub name_unique   : &'ent str,
    pub name_long     : &'ent str,
    pub name_simple   : &'ent str,
    pub name_absolute : &'ent str,
    pub name_short    : &'ent str,
    pub name_relative : &'ent str,
    pub kind          : UdbKind,
    pub language      : Option<Language>,
    pub library       : Option<UdbLibrary>,
    pub contents      : Option<&'ent str>,
    pub references    : Option<Vec<UdbReference>>,
    pub typetext      : Option<&'ent str>,
    pub freetext      : Option<&'ent str>,
    pub parameters    : Option<Vec<&'ent str>>,
    pub value         : Option<&'ent str>,
}

impl<'ent> Entity<'ent> {
    fn get_id(udb_entity: UdbEntity) -> i32 {
        unsafe {
            udbEntityId(udb_entity) as i32
        }
    }

    pub fn from_raw_list_ents(udb_list_ents: *mut UdbEntity, udb_count_ents: i32) -> Option<Vec<Self>> {
        let mut ret: Vec<Entity> = vec!();

        unsafe {
            for i in 0..udb_count_ents {
                ret.push(Entity{
                    id            : Entity::get_id(*udb_list_ents.offset(i as isize)),
                    name_unique   : "",
                    name_long     : "",
                    name_simple   : "",
                    name_absolute : "",
                    name_short    : "",
                    name_relative : "",
                    kind          : mem::uninitialized(),
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
