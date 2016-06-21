extern crate understand_sys;

use std::path::PathBuf;
use std::ffi::{CString, CStr};
use std::mem;

use language::Lang;
use status::Status;
use entity::Entity;

use understand_sys::{UdbEntity, udbDbOpen, udbDbLanguage, udbDbName, udbInfoBuild, UdbStatus,
UdbLanguage_, UdbLanguage, udbDbClose, udbListEntity, udbListEntityFree};

pub struct Db<'db> {
    pub name      : &'db str,
    pub path      : PathBuf,
    pub languages : Option<Vec<Lang>>,
    pub ents      : Option<Vec<Entity<'db>>>,
    pub version   : &'db str,
    pub status    : Status,
}

impl<'db> Db<'db> {
    pub fn new(path: &str) -> Self {
        unsafe {
            let udb_status = udbDbOpen(CString::new(path).unwrap().as_ptr());
            let udb_languages = udbDbLanguage();

            let ret = Db {
                name      : CStr::from_ptr(udbDbName()).to_str().unwrap(),
                path      : PathBuf::from(path),
                languages : Db::get_languages(udb_languages),
                ents      : Db::get_entities(),
                version   : CStr::from_ptr(udbInfoBuild()).to_str().unwrap(),
                status    : Db::get_status(udb_status),
            };

            udbDbClose();

            ret
            // TODO drop (free UDB) after all complete?
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

    fn get_languages(language: UdbLanguage) -> Option<Vec<Lang>> {
        let lang: u16 = language as u16;
        let mut ret: Vec<Lang> = vec!();
        if lang & UdbLanguage_::Udb_language_Ada as u16 != 0     { ret.push(Lang::Ada) };
        if lang & UdbLanguage_::Udb_language_Asm as u16 != 0     { ret.push(Lang::Asm) };
        if lang & UdbLanguage_::Udb_language_Basic as u16 != 0   { ret.push(Lang::Basic) };
        if lang & UdbLanguage_::Udb_language_C as u16 != 0       { ret.push(Lang::C) };
        if lang & UdbLanguage_::Udb_language_Cobol as u16 != 0   { ret.push(Lang::Cobol) };
        if lang & UdbLanguage_::Udb_language_CSharp as u16 != 0  { ret.push(Lang::CSharp) };
        if lang & UdbLanguage_::Udb_language_Fortran as u16 != 0 { ret.push(Lang::Fortran) };
        if lang & UdbLanguage_::Udb_language_Java as u16 != 0    { ret.push(Lang::Java) };
        if lang & UdbLanguage_::Udb_language_Jovial as u16 != 0  { ret.push(Lang::Jovial) };
        if lang & UdbLanguage_::Udb_language_Pascal as u16 != 0  { ret.push(Lang::Pascal) };
        if lang & UdbLanguage_::Udb_language_Plm as u16 != 0     { ret.push(Lang::Plm) };
        if lang & UdbLanguage_::Udb_language_Python as u16 != 0  { ret.push(Lang::Python) };
        if lang & UdbLanguage_::Udb_language_Verilog as u16 != 0 { ret.push(Lang::Verilog) };
        if lang & UdbLanguage_::Udb_language_Vhdl as u16 != 0    { ret.push(Lang::Vhdl) };
        if lang & UdbLanguage_::Udb_language_Web as u16 != 0     { ret.push(Lang::Web) };
        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }

    fn get_entities() -> Option<Vec<Entity<'db>>> {
        unsafe {
            let mut udb_list_ents: *mut UdbEntity = mem::uninitialized();
            let mut udb_count_ents: i32 = 0;

            udbListEntity(&mut udb_list_ents, &mut udb_count_ents);
            let list_ents: Option<Vec<Entity>> = Entity::from_raw_list_ents(udb_list_ents, udb_count_ents);
            udbListEntityFree(udb_list_ents);

            list_ents
        }
    }
}

/*
impl<'db> Drop for Db<'db> {
    fn drop(&mut self) {
    }
}
*/
