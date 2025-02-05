// TODO Tests with test database
extern crate understand_sys;
extern crate log;
extern crate time;

use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{DerefMut, Deref, Range};
use std::os::raw::c_char;

use understand_sys::{UdbReference, UdbEntity, UdbMetric, udbListEntityFree, udbEntityId,
                     udbEntityNameUnique, udbEntityNameLong, udbEntityNameSimple,
                     udbEntityNameShort, udbEntityKind, udbEntityLanguage, udbEntityLibrary,
                     udbEntityTypetext, udbEntityValue, udbEntityFreetext, udbListReference,
                     udbEntityNameAbsolute, udbEntityNameRelative, udbEntityRefs,
                     udbListReferenceFile, udbEntityParameters, udbMetricListEntity};

use db::Db;
use language::Language;
use library::Library;
use reference::ListReference;
use kind::{Kind, KindVec};
use metrics::Metric;


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
            Some(ListEntity {
                raw: raw,
                len: len as usize,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }

    /// Return raw pointer to UdbEntity.
    pub unsafe fn raw(&self) -> *mut UdbEntity {
        self.raw
    }

    /// Gets the number of entities that exist in the ListEntity.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.len < 1
    }

    /// Gets the Entity at the given index.
    pub fn get_index(&self, index: usize) -> Option<Entity> {
        unsafe {
            if index < self.len {
                Some(Entity::from_raw(*self.raw.offset(index as isize)))
            } else {
                None
            }
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
        self.iter().filter(|ent| kinds.locate(&ent.kind())).collect()
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

    pub unsafe fn raw(&self) -> UdbEntity {
        self.raw
    }

    /// Return the entity id. This is only valid until the db is changed.
    pub fn id(&self) -> i32 {
        unsafe { udbEntityId(self.raw) as i32 }
    }

    /// Return the entity unique name as string.
    pub fn name_unique(&self) -> String {
        unsafe { CStr::from_ptr(udbEntityNameUnique(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return the entity long name as string.
    /// If there is no long name the short name is returned.
    pub fn name_long(&self) -> String {
        unsafe { CStr::from_ptr(udbEntityNameLong(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return the entity simple name as string.
    pub fn name_simple(&self) -> String {
        unsafe { CStr::from_ptr(udbEntityNameSimple(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return the entity short name as string.
    pub fn name_short(&self) -> String {
        unsafe { CStr::from_ptr(udbEntityNameShort(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return the absolute name for file entity as string.
    /// TODO SIGSEGV sometimes! in for kraken:9e1c726
    /// use name_long!
    pub unsafe fn name_absolute(&self) -> Option<String> {
        if self.kind().is_file() {
            Some(CStr::from_ptr(udbEntityNameAbsolute(self.raw)).to_string_lossy().into_owned())
        } else {
            None
        }
    }

    /// Return the relative name for file entity as string.
    /// TODO SIGSEGV sometimes! in for kraken:9e1c726
    /// use name_long!
    pub unsafe fn name_relative(&self) -> Option<String> {
        if self.kind().is_file() {
            Some(CStr::from_ptr(udbEntityNameRelative(self.raw)).to_string_lossy().into_owned())
        } else {
            None
        }
    }

    /// Return the entity language.
    pub fn language(&self) -> Option<Language> {
        unsafe { Language::from_raw(udbEntityLanguage(self.raw)) }
    }

    /// Return the size of a temporary list of all metrics defined for the specified entity.
    pub fn metrics(&self) -> Vec<Metric> {
        unsafe {
            let mut metrics_mem: *mut UdbMetric = mem::uninitialized();
            let mut metrics: Vec<Metric> = Vec::new();
            let count_metrics = udbMetricListEntity(self.raw, &mut metrics_mem);

            for i in 0..count_metrics {
                metrics.push(Metric::from_raw(*metrics_mem.offset(i as isize)));
            }
            metrics
        }
    }

    /// Return the value of a metric for the specified entity.
    pub fn metric_value(&self, metric: &Metric) -> f64 {
        metric.value(self)
    }

    /// Return true if the specified metric is defined for the specified entity.
    pub fn is_defined_metric(&self, metric: &Metric) -> bool {
        metric.is_defined_entity(self)
    }

    /// Return the entity library. Never return NULL.
    pub fn library(&self) -> Library {
        unsafe { Library::from_raw(udbEntityLibrary(self.raw)) }
    }

    /// Return a string of the value associated with certain entities such as enumerators,
    /// initialized variables, default parameter values in function definitions and macros.
    pub fn value(&self) -> String {
        unsafe { CStr::from_ptr(udbEntityValue(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return Some(parameters prototype) if entity can have parameters.
    /// If showname is specified include the names of the parameters.
    pub fn parameters(&self, shownames: bool) -> Option<String> {
        unsafe {
            let mut parameters: *mut c_char = mem::uninitialized();
            let shownames_int: i32 = if shownames { 1 } else { 0 };
            if udbEntityParameters(self.raw, &mut parameters, shownames_int) > 0 {
                Some(CStr::from_ptr(parameters).to_string_lossy().into_owned())
            } else {
                None
            }
        }
    }

    /// Return the entity typetext as string.
    pub fn typetext(&self) -> Option<String> {
        unsafe {
            let raw: *const c_char = udbEntityTypetext(self.raw);
            if raw.is_null() {
                None
            } else {
                let res_str = CStr::from_ptr(raw).to_string_lossy().into_owned();
                if res_str.is_empty() {
                    None
                } else {
                    Some(res_str)
                }
            }
        }
    }

    /// Return debug information about CGraph(ControlFlow Graph) as string.
    pub fn cgraph(&self) -> Option<String> {
        unsafe {
            let cgraph_text_cstr = CString::new("CGraph").unwrap();
            let cgraph: String = CStr::from_ptr(udbEntityFreetext(self.raw,
                                                                  cgraph_text_cstr.as_ptr()))
                .to_string_lossy()
                .into_owned();
            if cgraph.is_empty() {
                None
            } else {
                Some(cgraph)
            }
        }
    }

    /// Return debug information about InitValue(init value of parameter) as string.
    pub fn init_value(&self) -> Option<String> {
        unsafe {
            let init_val_text_cstr = CString::new("InitValue").unwrap();
            let init_val: String = CStr::from_ptr(udbEntityFreetext(self.raw,
                                                                    init_val_text_cstr.as_ptr()))
                .to_string_lossy()
                .into_owned();
            if init_val.is_empty() {
                None
            } else {
                Some(init_val)
            }
        }
    }

    /// Return debug information about linkage(Linkage functions) as string.
    pub fn linkage(&self) -> Option<String> {
        unsafe {
            let linkage_text_cstr = CString::new("Linkage").unwrap();
            let linkage: String = CStr::from_ptr(udbEntityFreetext(self.raw,
                                                                   linkage_text_cstr.as_ptr()))
                .to_string_lossy()
                .into_owned();
            if linkage.is_empty() {
                None
            } else {
                Some(linkage)
            }
        }
    }

    /// Return debug information about Inline(inline function or not) as bool.
    /// TODO Test this function.
    pub unsafe fn inline(&self) -> bool {
        let inline_text_cstr = CString::new("Inline").unwrap();
        let inline: &CStr = CStr::from_ptr(udbEntityFreetext(self.raw, inline_text_cstr.as_ptr()));
        !inline.to_string_lossy().is_empty()
    }

    /// Return a list of Reference.
    pub fn references(&self) -> Option<ListReference> {
        unsafe {
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();
            let mut udb_count_refs: i32 = 0;

            udbListReference(self.raw, &mut udb_list_refs, &mut udb_count_refs);
            ListReference::from_raw(udb_list_refs, udb_count_refs)
        }
    }

    /// Return the entity kind.
    pub fn kind(&self) -> Kind {
        unsafe { Kind::from_raw(udbEntityKind(self.raw)) }
    }

    /// Check entity is file?
    pub fn is_file(&self) -> bool {
        self.kind().is_file()
    }

    /// Return true if the specified entity has any reference of the general kind specified by the
    /// list of references. Return true if the kind list is empty.
    pub fn locate_kinds_of_ref(&self, kinds: Vec<Kind>) -> bool {
        if let Some(refs) = self.references() {
            refs.iter().any(|reference| kinds.locate(&reference.kind()))
        } else {
            false
        }
    }

    /// Return an list of all references within file. If entity is not file than return empty
    /// ListReference.
    pub fn references_file(&self) -> Option<ListReference> {
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
    /// Maybe fixed by binding the string to a local variable.
    pub fn references_with_filter(&self,
                                  refkinds: Option<&str>,
                                  entkinds: Option<&str>,
                                  unique: bool)
                                  -> Option<ListReference> {
        unsafe {
            let mut udb_list_refs: *mut UdbReference = mem::uninitialized();
            let refkinds_cstr = CString::new(refkinds.unwrap()).unwrap();
            let entkinds_cstr = CString::new(entkinds.unwrap()).unwrap();
            let refkinds_raw = if refkinds.is_none() {
                ptr::null()
            } else {
                refkinds_cstr.as_ptr()
            };
            let entkinds_raw = if entkinds.is_none() {
                ptr::null()
            } else {
                entkinds_cstr.as_ptr()
            };
            let unique_raw: i32 = if unique { 1 } else { 0 };

            let udb_count_refs: i32 = udbEntityRefs(self.raw,
                                                    refkinds_raw,
                                                    entkinds_raw,
                                                    unique_raw,
                                                    &mut udb_list_refs);
            ListReference::from_raw(udb_list_refs, udb_count_refs)
        }
    }

    // pub fn references_filter_rs(&self,
    //                             refkinds: Option<&str>,
    //                             entkinds: Option<&str>)
    //                             -> Option<Vec<Reference>> {
    //     if let Some(refs) = self.references() {

    //         let mut refs_vec: Vec<Reference> = refs.iter().collect();
    //         if let Some(refkinds) = refkinds {
    //             let refkinds = Kind::parse(refkinds);
    //             refs_vec = refs_vec.into_iter()
    //                 .filter(|refer| refkinds.locate(refer.kind()))
    //                 .collect();
    //         }
    //         if let Some(entkinds) = entkinds {
    //             let entkinds = Kind::parse(entkinds);
    //             refs_vec = refs_vec.into_iter()
    //                 .filter(|refer| entkinds.locate(refer.kind()))
    //                 .collect();
    //         }
    //         if !refs_vec.is_empty() {
    //             Some(refs_vec)
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }
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

impl<'db> Deref for ListEntity<'db> {
    type Target = [Entity<'db>];

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(::std::slice::from_raw_parts(self.raw, self.len)) }
    }
}

impl<'db> DerefMut for ListEntity<'db> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(::std::slice::from_raw_parts_mut(self.raw, self.len)) }
    }
}

impl<'db> fmt::Debug for ListEntity<'db> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.raw, self.len)
    }
}

impl<'ents> fmt::Debug for Entity<'ents> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "raw: {raw:?}\nid: {id}\nname_unique: {n_unique}\nname_long: \
                {n_long}\nname_simple: {n_simple}\nname_short: {n_short}\nname_relative: \
                {n_relative}\nname_absolute: {n_absolute}\nkind: {kind}\nlibrary: \
                {lib}\nlanguage: {lang}\nvalue: {val}\ntypetext: {ttext:?}\ncgraph: \
                {cgraph:?}\nint_val: {init_val:?}\nlinkage: {linkage:?}",
               raw = self.raw,
               id = self.id(),
               n_unique = self.name_unique(),
               n_long = self.name_long(),
               n_simple = self.name_simple(),
               n_short = self.name_short(),
               // n_relative = self.name_relative().unwrap_or_default(),
               // n_absolute = self.name_absolute().unwrap_or_default(),
               n_relative = "",
               n_absolute = "",
               kind = self.kind().name_long(),
               lang = self.language().unwrap_or_default(),
               lib = self.library(),
               val = self.value(),
               ttext = self.typetext(),
               cgraph = self.cgraph(),
               init_val = self.init_value(),
               linkage = self.linkage())
    }
}

impl<'ents> fmt::Display for Entity<'ents> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.name_long(), self.kind())
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
