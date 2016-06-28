extern crate understand_sys;

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
    pub raw           : UdbEntity,
}

impl Entity {
    pub fn get_name_unique(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameUnique(self.raw)).to_string_lossy().into_owned()
        }
    }
    pub fn get_name_long(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameLong(self.raw)).to_string_lossy().into_owned()
        }
    }
    pub fn get_name_simple(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameSimple(self.raw)).to_string_lossy().into_owned()
        }
    }
    pub fn get_name_short(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameShort(self.raw)).to_string_lossy().into_owned()
        }
    }
    pub fn get_language(&self) -> Option<Language> {
        unsafe {
            Language::from_raw_language(udbEntityLanguage(self.raw))
        }
    }
    pub fn get_library(&self) -> Option<Library> {
        unsafe {
            Library::from_raw_library(udbEntityLibrary(self.raw))
        }
    }
    pub fn get_value(&self) -> Option<String> {
        unsafe {
            let value_raw: String = CStr::from_ptr(udbEntityValue(self.raw)).to_string_lossy().into_owned();
            match value_raw.is_empty() {
                false => Some(value_raw),
                true  => None,
            }
        }
    }
    pub fn get_typetext(&self) -> Option<String> {
        unsafe {
            let typetext_raw: String = CStr::from_ptr(udbEntityTypetext(self.raw)).to_string_lossy().into_owned();
            match typetext_raw.is_empty() {
                false => Some(typetext_raw),
                true  => None,
            }
        }
    }
    pub fn get_cgraph(&self) -> Option<String> {
        unsafe {
            let cgraph_text_raw = CString::new("CGraph").unwrap().as_ptr();
            let cgraph_raw: String = CStr::from_ptr(udbEntityFreetext(self.raw, cgraph_text_raw))
                .to_string_lossy().into_owned();
            match cgraph_raw.is_empty() {
                false => Some(cgraph_raw),
                true  => None,
            }
        }
    }
    pub fn get_references(&self) -> Option<Vec<Reference>> {
        unsafe {
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();
            let mut udb_count_refs: i32 = 0;

            udbListReference(self.raw, &mut udb_list_refs, &mut udb_count_refs);
            let list_refs: Option<Vec<Reference>> = Reference::from_raw_list_refs(udb_list_refs, udb_count_refs);
            udbListReferenceFree(udb_list_refs);

            list_refs

        /*
                pub fn udbListReference(entity : UdbEntity,
                                        refs   : *mut *mut UdbReference,
                                        items  : *mut c_int);

                pub fn udbListReferenceFree(refs: *mut UdbReference);
        */
        }
    }
    pub fn get_kind(&self) -> Kind {
        unsafe {
            Kind::from_raw_kind(udbEntityKind(self.raw))
        }
    }
    pub fn from_raw_entity(entity: UdbEntity) -> Self {
        unsafe {
            Entity{
                id  : udbEntityId(entity) as i32,
                raw : entity,
            }
        }
    }

    pub fn from_raw_list_ents(udb_list_ents: *mut UdbEntity, udb_count_ents: i32) -> Option<Vec<Self>> {
        let mut ret: Vec<Entity> = vec!();
        //let mut pb = ProgressBar::new(udb_count_ents as u64);
        //pb.message("Create entities: ");
        unsafe {
            for i in 0..udb_count_ents {
                //pb.inc();
                let entity: UdbEntity = *udb_list_ents.offset(i as isize);
                ret.push(Entity::from_raw_entity(entity));
            }
        }
        match ret.is_empty() {
            false => Some(ret),
            true  => None
        }
    }
}
