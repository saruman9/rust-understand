//! Metrics of `Entity`, the project, languages and etc.
//!
//! TODO Write docs.

use time;

use std::ffi::{CStr, CString};

use entity::Entity;
use language::Language;

use understand_sys::{UdbMetric, UdbMetricKind_, udbMetricDescription, udbMetricDescriptiveName,
                     udbMetricIsDefinedEntity, udbMetricKind, udbMetricLookup, udbMetricName,
                     udbMetricValue, udbMetricValueProject};

/// Structure of Metric.
pub struct Metric {
    raw: UdbMetric,
}

pub enum MetricKind {
    None,
    Integer,
    Real,
}

impl Metric {
    pub unsafe fn from_raw(raw: UdbMetric) -> Metric {
        debug!("Created Metric from {:?} at {}",
               raw,
               time::now().strftime("%M:%S.%f").unwrap());
        Metric { raw: raw }
    }

    /// Return the name of the specified metric, as a string.
    pub fn name(&self) -> String {
        unsafe { CStr::from_ptr(udbMetricName(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return the value of a metric for the specified entity.
    pub fn value(&self, ent: &Entity) -> f64 {
        unsafe { udbMetricValue(ent.raw(), self.raw) }
    }

    /// Return the value of a project metric.
    pub fn value_project(&self) -> f64 {
        unsafe { udbMetricValueProject(self.raw) }
    }

    /// Return the description of the specified metric, as a string.
    pub fn description(&self) -> String {
        unsafe { CStr::from_ptr(udbMetricDescription(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return the descriptive name of the specified metric, as a string.
    pub fn descriptive_name(&self) -> String {
        unsafe { CStr::from_ptr(udbMetricDescriptiveName(self.raw)).to_string_lossy().into_owned() }
    }

    /// Return true if the specified metric is defined for the specified entity.
    pub fn is_defined_entity(&self, ent: &Entity) -> bool {
        unsafe {
            if udbMetricIsDefinedEntity(self.raw, ent.raw()) == 1 {
                true
            } else {
                false
            }
        }
    }

    /// Return the value kind of a metric.
    pub fn kind(&self) -> MetricKind {
        unsafe {
            match udbMetricKind(self.raw) {
                UdbMetricKind_::Udb_mkind_NONE => MetricKind::None,
                UdbMetricKind_::Udb_mkind_Integer => MetricKind::Integer,
                UdbMetricKind_::Udb_mkind_Real => MetricKind::Real,
            }
        }
    }

    /// Lookup a metric by name.
    pub fn lookup(name: &str) -> Metric {
        let name_cstr = CString::new(name).unwrap();
        unsafe { Metric::from_raw(udbMetricLookup(name_cstr.as_ptr())) }
    }

    /// Return true if the specified metric is defined as a project metric for the specified
    /// language.
    ///
    /// TODO Implement method `raw` for `Language`.
    #[allow(unused_variables)]
    pub fn is_defined_project(&self, language: Language) -> bool {
        //     unsafe {
        //         if udbMetricIsDefinedProject(self.raw, language.raw()) == 1 {
        //             true
        //         } else {
        //             false
        //         }
        //     }
        unimplemented!()
    }

    // Return the size of a temporary, null-terminated list of all metrics defined for the specified
    // ent kinds filter.

    // pub fn udbMetricListKind(name    : *const c_char,
    //                          metrics : *mut *mut UdbMetric) -> c_int;

    // Return the size of a temporary, null-terminated list of all metrics defined for the specified
    // language.

    // pub fn udbMetricListLanguage(language : UdbLanguage,
    //                              metrics  : *mut *mut UdbMetric) -> c_int;

    // Return the size of a temporary, null-terminated list of all project metrics defined for the
    // specified language.

    // pub fn udbMetricListProject(language : UdbLanguage,
    //                             metrics  : *mut *mut UdbMetric) -> c_int;
}
