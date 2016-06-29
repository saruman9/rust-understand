extern crate understand_sys;

use std::path::PathBuf;
use std::ffi::{CString, CStr};
use std::mem;

use language::Language;
use status::Status;
use entity::Entity;

use understand_sys::{UdbEntity, udbDbOpen, udbDbLanguage, udbDbName,
udbInfoBuild, UdbStatus, UdbLanguage_, udbDbClose, udbListEntity,
udbListEntityFree};

pub struct Db {
    pub name: String,
    pub path: PathBuf,
    pub version: String,
    pub status: Status,
}

impl Db {
    pub fn open(path: &str) -> Self {
        unsafe {
            let udb_status = udbDbOpen(CString::new(path).unwrap().as_ptr());

            Db {
                name: CStr::from_ptr(udbDbName()).to_string_lossy().into_owned(),
                path: PathBuf::from(path),
                version: CStr::from_ptr(udbInfoBuild()).to_string_lossy().into_owned(),
                status: Db::get_status(udb_status),
            }
        }
    }
    pub fn get_entities(&self) -> Option<Vec<Entity>> {
        unsafe {
            let mut udb_list_ents: *mut UdbEntity = mem::uninitialized();
            let mut udb_count_ents: i32 = 0;

            udbListEntity(&mut udb_list_ents, &mut udb_count_ents);
            let list_ents: Option<Vec<Entity>> = Entity::from_raw_list_ents(udb_list_ents, udb_count_ents);
            // TODO dangling pointers?
            udbListEntityFree(udb_list_ents);

            list_ents
        }
    }
    /// Return a list of entities that match the specified name and kind.
    /// Empty strings for omit search pattern.
    pub fn lookup(&self, needle: &str, kind: &str) -> Option<Vec<Entity>> {
        let mut ents: Vec<Entity> = self.get_entities().unwrap();
        ents = ents.into_iter()
            .filter(|ent|
                    ent.get_name_long().find(needle).is_some())
            .filter(|ent|
            ent.get_kind().get_name_short().find(kind).is_some())
            .collect::<Vec<Entity>>();
        match ents.is_empty() {
            true => None,
            false => Some(ents),
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

    fn get_status(udb_status: UdbStatus) -> Status {
        match udb_status as u8 {
            0  => Status::Okay,
            1  => Status::DBAlreadyOpen,
            2  => Status::DBBusy,
            3  => Status::DBChanged,
            4  => Status::DBCorrupt,
            5  => Status::DBOldVersion,
            6  => Status::DBUnknownVersion,
            7  => Status::DBUnableCreate,
            8  => Status::DBUnableDelete,
            9  => Status::DBUnableModify,
            10 => Status::DBUnableOpen,
            11 => Status::DBUnableWrite,
            12 => Status::DemoAnotherDBOpen,
            13 => Status::DemoInvalid,
            14 => Status::DrawNoFont,
            15 => Status::DrawNoImage,
            16 => Status::DrawTooBig,
            17 => Status::DrawUnableCreateFile,
            18 => Status::DrawUnsupportedFile,
            19 => Status::LexerFileModified,
            20 => Status::LexerFileUnreadable,
            21 => Status::LexerUnsupportedLanguage,
            22 => Status::NoApiLicense,
            23 => Status::NoApiLicenseAda,
            24 => Status::NoApiLicenseC,
            25 => Status::NoApiLicenseCobol,
            26 => Status::NoApiLicenseFtn,
            27 => Status::NoApiLicenseJava,
            28 => Status::NoApiLicenseJovial,
            29 => Status::NoApiLicensePascal,
            30 => Status::NoApiLicensePlm,
            31 => Status::NoApiLicensePython,
            32 => Status::NoApiLicenseWeb,
            33 => Status::NoApiLicenseVhdl,
            34 => Status::NoApiLicenseVerilog,
            35 => Status::ReportUnableCreate,
            36 => Status::ReportUnableDelete,
            37 => Status::ReportUnableWrite,
            38 => Status::UserAbort,
            39 => Status::WrongProduct,
            _ => panic!("Unexpected status"),
        }
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        unsafe{
            udbDbClose();
        }
    }
}
