extern crate understand_sys;
extern crate log;
extern crate time;

use std::ffi::{CStr, CString};
use std::mem;
use std::fmt;
use std::marker::PhantomData;
use std::slice;
use std::ops::Range;

use db::Db;
use language::Language;
use library::Library;
//use kind::Kind;
//use reference::{Reference, ListReference};

use understand_sys::{UdbReference, UdbEntity, udbListEntityFree, udbEntityId, udbEntityNameUnique,
udbEntityNameLong, udbEntityNameSimple, udbEntityNameShort, udbEntityKind, udbEntityLanguage,
udbEntityLibrary, udbEntityTypetext, udbEntityValue, udbEntityFreetext, udbListReference,
udbEntityNameAbsolute, udbEntityNameRelative, udbEntityRefs, udbListReferenceFile};


/// Structure of Entity.
pub struct Entity<'ents> {
    raw: UdbEntity,
    _marker: PhantomData<&'ents UdbEntity>,
}

/// Opaque structure of list of entities.
pub struct ListEntity<'db> {
    raw: *mut UdbEntity,
    len: usize,
    _marker: PhantomData<&'db Db>,
    //pub list: Vec<Entity>,
    //pub old: bool,
}

/// An iterator over the Entity in list of entities.
pub struct EntityIter<'ents> {
    range: Range<usize>,
    ents: &'ents ListEntity<'ents>,
}

impl<'db> ListEntity<'db> {

    pub unsafe fn from_raw(raw: *mut UdbEntity, len: i32) -> ListEntity<'db> {
        debug!("Created ListEntity from {:?} with {} length at {}",
               raw,
               len,
               time::now().strftime("%S:%f").unwrap());

        ListEntity {
            raw: raw,
            len: len as usize,
            _marker: PhantomData,
        }
    }

    /// Gets the number of entity that exist in the ListEntity.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Gets the Entity at the given index.
    pub fn get_index(&self, index: usize) -> Option<Entity> {
        unsafe {
            if self.len > index {
                let new_array = slice::from_raw_parts(self.raw, self.len);
                Some(Entity::from_raw(new_array[index]))
            } else {
                None
            }
        }
    }

    /// Returns an iterator over the Entity in list of entities
    pub fn iter(&self) -> EntityIter {
        EntityIter {
            range: 0..self.len(),
            ents: self,
        }
    }
}

impl<'ents> Entity<'ents> {

    unsafe fn from_raw(raw: UdbEntity) -> Entity<'ents> {
        debug!("Created Entity from {:?} at {}",
               raw,
               time::now().strftime("%S:%f").unwrap());
        Entity {
            raw: raw,
            _marker: PhantomData,
        }
    }

    /// Return the entity id. This is only valid until the db is changed.
    pub fn id(&self) -> i32 {
        unsafe {
            udbEntityId(self.raw) as i32
        }
    }

    /// Return the entity unique name as String.
    pub fn name_unique(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameUnique(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the entity long name as String. If there is no long name the short name is returned.
    pub fn name_long(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameLong(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the entity simple name as String.
    pub fn name_simple(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameSimple(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the entity short name as String.
    pub fn name_short(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameShort(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the absolute name for file entity as String. May be error - segmentation fault.
    pub unsafe fn name_absolute(&self) -> String {
            CStr::from_ptr(udbEntityNameAbsolute(self.raw)).to_string_lossy().into_owned()
    }

    /// Return the relative name for file entity as String.
    pub fn name_relative(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameRelative(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the entity language.
    pub fn language(&self) -> Option<Language> {
        unsafe {
            Language::from_raw(udbEntityLanguage(self.raw))
        }
    }

    /// Return the entity library.
    pub fn library(&self) -> Option<Library> {
        unsafe {
            Library::from_raw(udbEntityLibrary(self.raw))
        }
    }

    /*
    /// Return a string of the value associated with certain entities such as enumerators,
    /// initialized variables, default parameter values in function definitions and macros.
    pub fn get_value(&self) -> Option<String> {
        unsafe {
            let value_raw: String = CStr::from_ptr(udbEntityValue(self.raw))
                .to_string_lossy().into_owned();
            match value_raw.is_empty() {
                false => Some(value_raw),
                true  => None,
            }
        }
    }
    /// Return the entity typetext as a string.
    pub fn get_typetext(&self) -> Option<String> {
        unsafe {
            let typetext_raw: String = CStr::from_ptr(udbEntityTypetext(self.raw))
                .to_string_lossy().into_owned();
            match typetext_raw.is_empty() {
                false => Some(typetext_raw),
                true  => None,
            }
        }
    }
    /// Return debug information about CGraph(ControlFlow Graph)
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
    /// Return an allocated list of all references within file.
    pub fn get_references_file(&self) -> Option<ListReference> {
        unsafe {
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();
            let mut udb_count_refs: i32 = 0;

            udbListReferenceFile(self.raw, &mut udb_list_refs, &mut udb_count_refs);

            Reference::from_raw_list_refs(udb_list_refs, udb_count_refs)
        }
    }
    /// Return a vec of references, using the refkinds and/or the entkinds specified.
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
    */
}

impl<'ents, 'iter> IntoIterator for &'iter ListEntity<'ents> {
    type Item = Entity<'iter>;
    type IntoIter = EntityIter<'iter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'ents> Iterator for EntityIter<'ents> {
    type Item = Entity<'ents>;

    fn next(&mut self) -> Option<Entity<'ents>> {
        self.range.next().and_then(|i| self.ents.get_index(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    fn count(self) -> usize {
        self.size_hint().0
    }
}

impl<'ents> DoubleEndedIterator for EntityIter<'ents> {
    fn next_back(&mut self) -> Option<Entity<'ents>> {
        self.range.next_back().and_then(|i| self.ents.get_index(i))
    }
}

impl<'db> fmt::Debug for ListEntity<'db> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.raw, self.len)
    }
}

impl<'ents> fmt::Debug for Entity<'ents> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.raw)
    }
}

impl<'ents> fmt::Display for Entity<'ents> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\t{}",
               self.name_long(),
               self.language().unwrap_or(Language::NONE))
    }
}

impl<'db> Drop for ListEntity<'db> {
    fn drop(&mut self) {
        debug!("Dropped ListEntity {:?} at {}",
               self.raw,
               time::now().strftime("%S:%f").unwrap());

        unsafe { udbListEntityFree(self.raw) };
    }
}
