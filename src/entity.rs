extern crate understand_sys;
extern crate pbr;

use std::ffi::{CStr, CString};
use std::mem;

use language::Language;
use kind::Kind;
use library::Library;
use reference::Reference;

use self::pbr::ProgressBar;

use understand_sys::{UdbReference, UdbEntity, udbEntityId, udbEntityNameUnique,
udbEntityNameLong, udbEntityNameSimple, udbEntityNameShort, udbEntityKind,
udbEntityLanguage, udbEntityLibrary, udbEntityTypetext, udbEntityValue,
udbEntityFreetext, udbListReference, udbListReferenceFree};


#[derive(Clone)]
pub struct Entity {
    pub id            : i32,
    pub name_unique   : String,
    pub name_long     : String,
    pub name_simple   : String,
    pub name_short    : String,
    pub kind          : Kind,
    pub language      : Option<Language>,
    pub library       : Option<Library>,
    // TODO Contents too large
    pub contents      : Option<String>,
    pub references    : Option<Vec<Reference>>,
    pub typetext      : Option<String>,
    pub cgraph        : Option<String>,
    // TOOD needed?
    pub parameters    : Option<Vec<String>>,
    pub value         : Option<String>,
    // TODO Remove?
    pub name_absolute : Option<String>,
    pub name_relative : Option<String>,
}

impl Entity {
    pub fn from_raw_entity(entity: UdbEntity) -> Self {
        unsafe {
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
            let library: Option<Library> = Library::from_raw_library(udbEntityLibrary(entity));

            let typetext_raw: String = CStr::from_ptr(udbEntityTypetext(entity)).to_string_lossy().into_owned();
            let typetext: Option<String>;
            if typetext_raw.is_empty() {
                typetext = None;
            } else {
                typetext = Some(typetext_raw);
            }

            let value_raw: String = CStr::from_ptr(udbEntityValue(entity)).to_string_lossy().into_owned();
            let value: Option<String>;
            if value_raw.is_empty() {
                value = None;
            } else {
                value = Some(value_raw);
            }

            let cgraph_text_raw = CString::new("CGraph").unwrap().as_ptr();
            let cgraph_raw: String = CStr::from_ptr(udbEntityFreetext(entity, cgraph_text_raw))
                .to_string_lossy()
                .into_owned();
            let cgraph: Option<String>;
            if cgraph_raw.is_empty() {
                cgraph = None;
            } else {
                cgraph = Some(cgraph_raw);
            }

            let references = Entity::get_references(entity);

            Entity{
                id            : id,
                name_unique   : name_unique,
                name_long     : name_long,
                name_simple   : name_simple,
                name_short    : name_short,
                name_absolute : None,
                name_relative : None,
                kind          : kind,
                language      : language,
                library       : library,
                contents      : None,
                references    : references,
                typetext      : typetext,
                cgraph        : cgraph,
                parameters    : None,
                value         : value,
            }
        }
    }

    pub fn from_raw_list_ents(udb_list_ents: *mut UdbEntity, udb_count_ents: i32) -> Option<Vec<Self>> {
        let mut ret: Vec<Entity> = vec!();
        let mut pb = ProgressBar::new(udb_count_ents as u64);
        pb.message("Create entities: ");
        unsafe {
            for i in 0..udb_count_ents {
                pb.inc();
                let entity: UdbEntity = *udb_list_ents.offset(i as isize);
                ret.push(Entity::from_raw_entity(entity));
            }
        }
        match ret.is_empty() {
            false => Some(ret),
            true  => None
        }
    }

    fn get_references(raw_entity: UdbEntity) -> Option<Vec<Reference>> {
        unsafe{
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();
            let mut udb_count_refs: i32 = 0;

            udbListReference(raw_entity, &mut udb_list_refs, &mut udb_count_refs);
            let list_refs: Option<Vec<Reference>> = Reference::from_raw_list_refs(udb_list_refs, udb_count_refs);
            udbListReferenceFree(udb_list_refs);

            list_refs
        }
        /*
                pub fn udbListReference(entity : UdbEntity,
                                        refs   : *mut *mut UdbReference,
                                        items  : *mut c_int);

                pub fn udbListReferenceFree(refs: *mut UdbReference);
        */

    }
}
