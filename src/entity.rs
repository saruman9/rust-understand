extern crate understand_sys;

use std::ffi::CStr;

use language::Language;
use kind::Kind;

use understand_sys::{UdbReference, UdbLibrary, UdbEntity, udbEntityId,
udbEntityNameUnique, udbEntityNameLong, udbEntityNameSimple, udbEntityNameShort,
udbEntityKind, udbEntityLanguage};


#[derive(Clone)]
pub struct Entity {
    pub id            : i32,
    pub name_unique   : String,
    pub name_long     : String,
    pub name_simple   : String,
    pub name_short    : String,
    pub kind          : Kind,
    pub language      : Option<Language>,
    pub library       : Option<UdbLibrary>,
    pub contents      : Option<String>,
    pub references    : Option<Vec<UdbReference>>,
    pub typetext      : Option<String>,
    pub freetext      : Option<String>,
    pub parameters    : Option<Vec<String>>,
    pub value         : Option<String>,
    // TODO Remove?
    pub name_absolute : Option<String>,
    pub name_relative : Option<String>,
}

impl Entity {
    pub fn from_raw_list_ents(udb_list_ents: *mut UdbEntity, udb_count_ents: i32) -> Option<Vec<Self>> {
        let mut ret: Vec<Entity> = vec!();
        unsafe {
            for i in 0..udb_count_ents {
                let entity: UdbEntity = *udb_list_ents.offset(i as isize);
                let id: i32 = udbEntityId(entity) as i32;
                let name_unique: String = CStr::from_ptr(
                    udbEntityNameUnique(entity))
                    .to_string_lossy().into_owned();
                let name_long: String = CStr::from_ptr(
                    udbEntityNameLong(entity))
                    .to_string_lossy().into_owned();
                let name_simple: String = CStr::from_ptr(
                    udbEntityNameSimple(entity))
                    .to_string_lossy().into_owned();
                let name_short: String = CStr::from_ptr(
                    udbEntityNameShort(entity))
                    .to_string_lossy().into_owned();
                let kind: Kind = Kind::from_raw_kind(udbEntityKind(entity));
                /*
                // Don't work API - segmentation fault
                let name_absolute: &CStr = CStr::from_ptr(
                    udbEntityNameAbsolute(entity));
                let name_relative: Option<&str> = CStr::from_ptr(
                    udbEntityNameRelative(entity))
                    .to_str().ok();
                */
                let language: Option<Language> = Language::from_raw_language(udbEntityLanguage(entity));

                ret.push(Entity{
                    id            : id,
                    name_unique   : name_unique,
                    name_long     : name_long,
                    name_simple   : name_simple,
                    name_short    : name_short,
                    name_absolute : None,
                    name_relative : None,
                    kind          : kind,
                    language      : language,
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
