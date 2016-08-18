extern crate understand_sys;
extern crate log;
extern crate time;

use std::path::PathBuf;
use std::ffi::{CString, CStr};
use std::mem;

use language::Language;
use errors::StatusError;
use entity::{Entity, ListEntity};

use understand_sys::{UdbEntity, udbDbOpen, udbDbLanguage, udbDbName, udbInfoBuild, UdbStatus,
UdbLanguage_, udbDbClose, udbListEntity, udbLookupEntityByUniquename, udbListFile, udbLookupEntity};

pub struct Db {
    pub path: PathBuf,
}

impl Db {
    pub fn open(path: &str) -> Result<Self, StatusError> {
        unsafe {
            debug!("Created Db at {}", time::now().strftime("%S:%f").unwrap());
            let udb_status: StatusError = Db::get_status(udbDbOpen(
                CString::new(path).unwrap().as_ptr()));

            match udb_status {
                StatusError::Okay => Ok(Db { path: PathBuf::from(path) }),
                _ => Err(udb_status),
            }
        }
    }
    pub fn get_name(&self) -> String {
        unsafe{ CStr::from_ptr(udbDbName()).to_string_lossy().into_owned() }
    }
    pub fn get_version(&self) -> String {
        unsafe{ CStr::from_ptr(udbInfoBuild()).to_string_lossy().into_owned() }
    }
    pub fn get_entities(&self) -> Option<ListEntity> {
        unsafe {
            let mut udb_list_ents: *mut UdbEntity = mem::uninitialized();
            let mut udb_count_ents: i32 = 0;

            udbListEntity(&mut udb_list_ents, &mut udb_count_ents);
            Entity::from_raw_list_ents(udb_list_ents, udb_count_ents)
        }
    }
    /// Lookup and return an allocated list of entities by name and kind, if specified.
    /// !SLOWER! then lookup on Rust
    pub fn lookup_entity(&self, name: &str, kind: &str, search_in_shortname: bool)
                         -> Option<ListEntity>{
        unsafe {
            let mut udb_list_ents: *mut UdbEntity = mem::uninitialized();
            let mut udb_count_ents: i32 = 0;

            let search_in_shortname_int = if search_in_shortname { 1 } else { 0 } ;
            udbLookupEntity(CString::new(name).unwrap().as_ptr(),
                            CString::new(kind).unwrap().as_ptr(),
                            search_in_shortname_int,
                            &mut udb_list_ents,
                            &mut udb_count_ents);
            Entity::from_raw_list_ents(udb_list_ents, udb_count_ents)
        }
    }
    /// Return a temporary list of all analyzed file entities.
    pub fn get_files(&self) -> Option<ListEntity> {
        unsafe {
            let mut udb_list_files: *mut UdbEntity = mem::uninitialized();
            let mut udb_count_files: i32 = 0;

            udbListFile(&mut udb_list_files, &mut udb_count_files);
            Entity::from_raw_list_ents(udb_list_files, udb_count_files)
        }
    }
    pub fn lookup_file(&self, needle: &str) -> Option<ListEntity> {
        let files: Option<ListEntity> = self.get_files();
        files.map(|mut files| {
            files.list = files.list.clone().into_iter()
                .filter(|file|
                        file.get_name_long().find(needle).is_some())
                .collect();
            files.old = true;
            files
        })
    }
    /// Return a list of entities that match the specified name and kind.
    /// Empty strings for omit search pattern.
    pub fn lookup(&self, needle: &str, kind: &str) -> Option<ListEntity> {
        let ents: Option<ListEntity> = self.get_entities();
        ents.map(|mut ents| {
            ents.list = ents.list.clone().into_iter()
                .filter(|ent|
                        ent.get_name_long().find(needle).is_some())
                .filter(|ent|
                        ent.get_kind().get_name_short().find(kind).is_some())
                .collect();
            ents.old = true;
            ents
        })
    }
    /// Lookup an entity by unique name.
    pub fn lookup_by_name_unique(needle: &str) -> Entity {
        unsafe {
            Entity::from_raw_entity(
                udbLookupEntityByUniquename(CString::new(needle).unwrap().as_ptr())
            )
        }
    }
    pub fn get_languages(&self) -> Option<Vec<Language>> {
        unsafe {
            let lang: u16 = udbDbLanguage() as u16;
            let mut ret: Vec<Language> = vec!();
            if lang & UdbLanguage_::Udb_language_Ada as u16 != 0     { ret.push(Language::Ada) };
            if lang & UdbLanguage_::Udb_language_Asm as u16 != 0     { ret.push(Language::Asm) };
            if lang & UdbLanguage_::Udb_language_Basic as u16 != 0   { ret.push(Language::Basic) };
            if lang & UdbLanguage_::Udb_language_C as u16 != 0       { ret.push(Language::C) };
            if lang & UdbLanguage_::Udb_language_Cobol as u16 != 0   { ret.push(Language::Cobol) };
            if lang & UdbLanguage_::Udb_language_CSharp as u16 != 0  { ret.push(Language::CSharp) };
            if lang & UdbLanguage_::Udb_language_Fortran as u16 != 0 { ret.push(Language::Fortran) };
            if lang & UdbLanguage_::Udb_language_Java as u16 != 0    { ret.push(Language::Java) };
            if lang & UdbLanguage_::Udb_language_Jovial as u16 != 0  { ret.push(Language::Jovial) };
            if lang & UdbLanguage_::Udb_language_Pascal as u16 != 0  { ret.push(Language::Pascal) };
            if lang & UdbLanguage_::Udb_language_Plm as u16 != 0     { ret.push(Language::Plm) };
            if lang & UdbLanguage_::Udb_language_Python as u16 != 0  { ret.push(Language::Python) };
            if lang & UdbLanguage_::Udb_language_Verilog as u16 != 0 { ret.push(Language::Verilog) };
            if lang & UdbLanguage_::Udb_language_Vhdl as u16 != 0    { ret.push(Language::Vhdl) };
            if lang & UdbLanguage_::Udb_language_Web as u16 != 0     { ret.push(Language::Web) };
            if ret.is_empty() {
                None
            } else {
                Some(ret)
            }
        }
    }

    fn get_status(udb_status: UdbStatus) -> StatusError {
        match udb_status as u8 {
            0  => StatusError::Okay,
            1  => StatusError::DBAlreadyOpen,
            2  => StatusError::DBBusy,
            3  => StatusError::DBChanged,
            4  => StatusError::DBCorrupt,
            5  => StatusError::DBOldVersion,
            6  => StatusError::DBUnknownVersion,
            7  => StatusError::DBUnableCreate,
            8  => StatusError::DBUnableDelete,
            9  => StatusError::DBUnableModify,
            10 => StatusError::DBUnableOpen,
            11 => StatusError::DBUnableWrite,
            12 => StatusError::DemoAnotherDBOpen,
            13 => StatusError::DemoInvalid,
            14 => StatusError::DrawNoFont,
            15 => StatusError::DrawNoImage,
            16 => StatusError::DrawTooBig,
            17 => StatusError::DrawUnableCreateFile,
            18 => StatusError::DrawUnsupportedFile,
            19 => StatusError::LexerFileModified,
            20 => StatusError::LexerFileUnreadable,
            21 => StatusError::LexerUnsupportedLanguage,
            22 => StatusError::NoApiLicense,
            23 => StatusError::NoApiLicenseAda,
            24 => StatusError::NoApiLicenseC,
            25 => StatusError::NoApiLicenseCobol,
            26 => StatusError::NoApiLicenseFtn,
            27 => StatusError::NoApiLicenseJava,
            28 => StatusError::NoApiLicenseJovial,
            29 => StatusError::NoApiLicensePascal,
            30 => StatusError::NoApiLicensePlm,
            31 => StatusError::NoApiLicensePython,
            32 => StatusError::NoApiLicenseWeb,
            33 => StatusError::NoApiLicenseVhdl,
            34 => StatusError::NoApiLicenseVerilog,
            35 => StatusError::ReportUnableCreate,
            36 => StatusError::ReportUnableDelete,
            37 => StatusError::ReportUnableWrite,
            38 => StatusError::UserAbort,
            39 => StatusError::WrongProduct,
            _ => panic!("Unexpected status"),
        }
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        debug!("Dropped Db at {}", time::now().strftime("%S:%f").unwrap());
        unsafe{
            udbDbClose();
        }
    }
}
