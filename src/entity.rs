// TODO Tests with test database
extern crate understand_sys;
extern crate log;
extern crate time;

use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Range;

use understand_sys::{UdbReference, UdbEntity, udbListEntityFree, udbEntityId, udbEntityNameUnique,
udbEntityNameLong, udbEntityNameSimple, udbEntityNameShort, udbEntityKind, udbEntityLanguage,
udbEntityLibrary, udbEntityTypetext, udbEntityValue, udbEntityFreetext, udbListReference,
udbEntityNameAbsolute, udbEntityNameRelative, udbEntityRefs, udbListReferenceFile};

use db::Db;
use language::Language;
use library::Library;
use reference::ListReference;
use kind::{Kind, KindList};


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
}

/// An iterator over the Entity in list of entities.
pub struct EntityIter<'ents> {
    range: Range<usize>,
    ents: &'ents ListEntity<'ents>,
}

impl<'db> ListEntity<'db> {

    /// Create ListEntity from raw - *mut UdbEntity.
    pub unsafe fn from_raw(raw: *mut UdbEntity, len: i32) -> Option<ListEntity<'db>> {
        debug!("Created ListEntity from {:?} with {} length at {}",
               raw,
               len,
               time::now().strftime("%M:%S.%f").unwrap());

        if len > 0 {
            Some(
                ListEntity {
                    raw: raw,
                    len: len as usize,
                    _marker: PhantomData,
                }
            )
        } else { None }
    }

    /// Return raw pointer to UdbEntity.
    pub unsafe fn raw(&self) -> *mut UdbEntity {
        self.raw
    }

    /// Gets the number of entities that exist in the ListEntity.
    pub fn len(&self) -> usize { self.len }

    /// Gets the Entity at the given index.
    pub fn get_index(&self, index: usize) -> Option<Entity> {
        unsafe {
            if index < self.len {
                Some(Entity::from_raw(*self.raw.offset(index as isize)))
            } else { None }
        }
    }

    /// Return an iterator over the Entity in list of entities
    pub fn iter(&self) -> EntityIter {
        EntityIter {
            range: 0..self.len(),
            ents: self,
        }
    }

    /// Filter the specified list of entities, using the kinds specified, and return a new Vec.
    pub fn filter_by_kinds(&self, kinds: Vec<Kind>) -> Vec<Entity> {
        self.iter().filter(|ent| kinds.locate(ent.kind())).collect()
    }
}

impl<'ents> Entity<'ents> {

    /// Create Entity from raw - UdbEntity.
    pub unsafe fn from_raw(raw: UdbEntity) -> Entity<'ents> {
        debug!("Created Entity from {:?} at {}",
               raw,
               time::now().strftime("%M:%S.%f").unwrap());
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

    /// Return the entity unique name as string.
    pub fn name_unique(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameUnique(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the entity long name as string.
    /// If there is no long name the short name is returned.
    pub fn name_long(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameLong(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the entity simple name as string.
    pub fn name_simple(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameSimple(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the entity short name as string.
    pub fn name_short(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityNameShort(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the absolute name for file entity as string.
    pub fn name_absolute(&self) -> Option<String> {
        if self.kind().is_file() {
            unsafe {
                Some(CStr::from_ptr(udbEntityNameAbsolute(self.raw)).to_string_lossy().into_owned())
            }
        } else { None }
    }

    /// Return the relative name for file entity as string.
    pub fn name_relative(&self) -> Option<String> {
        if self.kind().is_file() {
            unsafe {
                Some(CStr::from_ptr(udbEntityNameRelative(self.raw)).to_string_lossy().into_owned())
            }
        } else { None }
    }

    /// Return the entity language.
    pub fn language(&self) -> Option<Language> {
        unsafe {
            Language::from_raw(udbEntityLanguage(self.raw))
        }
    }

    /// Return the entity library. Never return NULL.
    pub fn library(&self) -> Library {
        unsafe {
            Library::from_raw(udbEntityLibrary(self.raw))
        }
    }

    /// Return a string of the value associated with certain entities such as enumerators,
    /// initialized variables, default parameter values in function definitions and macros.
    pub fn value(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityValue(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return the entity typetext as string.
    pub fn typetext(&self) -> String {
        unsafe {
            CStr::from_ptr(udbEntityTypetext(self.raw)).to_string_lossy().into_owned()
        }
    }

    /// Return debug information about CGraph(ControlFlow Graph) as string.
    pub fn cgraph(&self) -> String {
        unsafe {
            let cgraph_text_raw = CString::new("CGraph").unwrap().as_ptr();
            CStr::from_ptr(udbEntityFreetext(self.raw, cgraph_text_raw)).to_string_lossy().into_owned()
        }
    }

    /// Return a list of Reference.
    pub fn references(&self) -> ListReference {
        unsafe {
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();
            let mut udb_count_refs: i32 = 0;

            udbListReference(self.raw, &mut udb_list_refs, &mut udb_count_refs);
            ListReference::from_raw(udb_list_refs, udb_count_refs)
        }
    }

    /// Return the entity kind.
    pub fn kind(&self) -> Kind {
        unsafe {
            Kind::from_raw(udbEntityKind(self.raw))
        }
    }

    /// Return true if the specified entity has any reference of the general kind specified by the
    /// list of references. Return true if the kind list is empty.
    pub fn locate_kinds_of_ref(&self, kinds: Vec<Kind>) -> bool {
        let refs: ListReference = self.references();
        for reference in &refs {
            if kinds.locate(&reference.kind()) { return true }
        }
        false
    }

    /// Return an list of all references within file. If entity is not file than return empty
    /// ListReference.
    pub fn references_file(&self) -> ListReference {
        unsafe {
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();
            let mut udb_count_refs: i32 = mem::uninitialized();

            udbListReferenceFile(self.raw, &mut udb_list_refs, &mut udb_count_refs);
            ListReference::from_raw(udb_list_refs, udb_count_refs)
        }
    }

    /// Return an list of references, using the refkinds and/or the entkinds specified. Set unique
    /// to true to return only the first matching reference to each unique entity. Set to false
    /// otherwise.
    /// TODO Rewrite on Rust for much speed?
    /// !!! Don't work udbEntityRefs or I'm stupid.
    pub fn references_with_filter(&self,
                                  refkinds: Option<&str>,
                                  entkinds: Option<&str>,
                                  unique: bool) -> ListReference {
        unsafe {
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();
            let refkinds_raw = if refkinds.is_none() { ptr::null() } else {
                CString::new(refkinds.unwrap()).unwrap().as_ptr()
            };
            let entkinds_raw = if entkinds.is_none() { ptr::null() } else {
                CString::new(entkinds.unwrap()).unwrap().as_ptr()
            };
            let unique_raw: i32 = if unique { 1 } else { 0 };
            debug!("refkinds: {:?}; entkinds: {:?}, unique: {}", refkinds_raw, entkinds_raw, unique_raw);

            let udb_count_refs: i32 = udbEntityRefs(self.raw,
                                                    refkinds_raw,
                                                    entkinds_raw,
                                                    unique_raw,
                                                    &mut udb_list_refs);
            ListReference::from_raw(udb_list_refs, udb_count_refs)
        }
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

impl<'ents, 'iter> IntoIterator for &'iter ListEntity<'ents> {
    type Item = Entity<'iter>;
    type IntoIter = EntityIter<'iter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
        write!(f, "\
raw: {raw:?}\n\
id: {id}\n\
name_unique: {n_unique}\n\
name_long: {n_long}\n\
name_simple: {n_simple}\n\
name_short: {n_short}\n\
name_relative: {n_relative}\n\
name_absolute: {n_absolute}\n\
kind: {kind}\n\
library: {lib}\n\
language: {lang}\n\
value: {val}\n\
typetext: {ttext}\n\
cgraph: {freetext}\n\
================================================================================",
               raw=self.raw,
               id=self.id(),
               n_unique=self.name_unique(),
               n_long=self.name_long(),
               n_simple=self.name_simple(),
               n_short=self.name_short(),
               n_relative=self.name_relative().unwrap_or_default(),
               n_absolute=self.name_absolute().unwrap_or_default(),
               kind=self.kind().name_long(),
               lang=self.language().unwrap_or_default(),
               lib=self.library(),
               val=self.value(),
               ttext=self.typetext(),
               freetext=self.cgraph())
    }
}

impl<'ents> fmt::Display for Entity<'ents> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}",
               self.name_long(),
               self.kind())
    }
}

impl<'db> Drop for ListEntity<'db> {
    fn drop(&mut self) {
        debug!("Dropped ListEntity {:?} at {}",
               self.raw,
               time::now().strftime("%M:%S.%f").unwrap());

        unsafe { udbListEntityFree(self.raw) };
    }
}
