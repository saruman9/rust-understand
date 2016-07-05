extern crate understand_sys;

use std::ffi::{CStr, CString};
use std::mem;
use std::fmt;

use language::Language;
use kind::Kind;
use library::Library;
use reference::{Reference, ListReference};

use understand_sys::{UdbReference, UdbEntity, udbListEntityFree, udbEntityId, udbEntityNameUnique,
udbEntityNameLong, udbEntityNameSimple, udbEntityNameShort, udbEntityKind, udbEntityLanguage,
udbEntityLibrary, udbEntityTypetext, udbEntityValue, udbEntityFreetext, udbListReference,
udbEntityNameAbsolute, udbEntityNameRelative, udbEntityRefs, udbListReferenceFree};


#[derive(Clone, Debug)]
pub struct Entity {
    pub raw: UdbEntity,
}

pub struct ListEntity {
    pub raw: *mut UdbEntity,
    pub list: Vec<Entity>,
    pub old: bool,
}

impl Entity {
    pub fn from_raw_entity(entity: UdbEntity) -> Self {
            Entity{ raw: entity }
    }
    pub fn from_raw_list_ents(udb_list_ents: *mut UdbEntity, udb_count_ents: i32)
                              -> Option<ListEntity> {
        let mut ret: Vec<Entity> = vec!();
        unsafe {
            for i in 0..udb_count_ents {
                let entity: UdbEntity = *udb_list_ents.offset(i as isize);
                ret.push(Entity::from_raw_entity(entity));
            }
        }
        match ret.is_empty() {
            false => Some(ListEntity {
                raw: udb_list_ents,
                list: ret,
                old: false,
            }),
            true => None,
        }
    }
    /// Return the entity id. This is only valid until the db is changed.
    pub fn get_id(&self) -> i32 {
        unsafe {
            udbEntityId(self.raw) as i32
        }
    }
    /// Return the entity unique name as String.
    pub fn get_name_unique(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameUnique(self.raw)).to_string_lossy().into_owned()
        }
    }
    /// Return the entity long name as String. If there is no long name the short name is returned.
    pub fn get_name_long(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameLong(self.raw)).to_string_lossy().into_owned()
        }
    }
    /// Return the entity simple name as String.
    pub fn get_name_simple(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameSimple(self.raw)).to_string_lossy().into_owned()
        }
    }
    /// Return the entity short name as String.
    pub fn get_name_short(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameShort(self.raw)).to_string_lossy().into_owned()
        }
    }
    /// Return the absolute name for file entity as String. May be error - segmentation fault.
    pub unsafe fn get_name_absolute(&self) -> String {
            CStr::from_ptr(udbEntityNameAbsolute(self.raw)).to_string_lossy().into_owned()
    }
    /// Return the relative name for file entity as String.
    pub fn get_name_relative(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameRelative(self.raw)).to_string_lossy().into_owned()
        }
    }
    /// Return the entity language.
    pub fn get_language(&self) -> Option<Language> {
        unsafe {
            Language::from_raw_language(udbEntityLanguage(self.raw))
        }
    }
    /// Return the entity library.
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
    /// Return a vec of all references for entity.
    pub fn get_references(&self) -> Option<ListReference> {
        let list_refs: Option<ListReference>;
        unsafe {
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();
            let mut udb_count_refs: i32 = 0;

            udbListReference(self.raw, &mut udb_list_refs, &mut udb_count_refs);
            list_refs = Reference::from_raw_list_refs(udb_list_refs, udb_count_refs);

            list_refs
        }
    }
    /// Return a vec of references, using the refkinds and/or
    /// the entkinds specified.
    pub fn get_references_with_filter(&self,
                                      refkinds: &str,
                                      entkinds: &str,
                                      unique: i32) -> Option<ListReference> {
        let list_refs: Option<ListReference>;

        unsafe {
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();

            let refkinds_raw = CString::new(refkinds).unwrap().as_ptr();
            let entkinds_raw = CString::new(entkinds).unwrap().as_ptr();

            let udb_count_refs: i32 = udbEntityRefs(self.raw,
                                                    refkinds_raw,
                                                    entkinds_raw,
                                                    unique,
                                                    &mut udb_list_refs);
            list_refs = Reference::from_raw_list_refs(udb_list_refs, udb_count_refs);
        }
        list_refs
    }
    /// Return the entity kind.
    pub fn get_kind(&self) -> Kind {
        unsafe {
            Kind::from_raw_kind(udbEntityKind(self.raw))
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\t{} - {}",
               self.get_name_long(),
               self.get_language().unwrap_or(Language::NONE),
               self.get_kind().get_name_short())
    }
}

impl Drop for ListEntity {
    fn drop(&mut self) {
        unsafe { udbListEntityFree(self.raw) };
    }
}
