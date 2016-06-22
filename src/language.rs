extern crate understand_sys;

use std::fmt;

use understand_sys::{UdbLanguage_, UdbLanguage};


#[derive(Clone)]
pub enum Lang {
    NONE,
    ALL,
    Ada,
    Asm,
    Basic,
    C,
    Cobol,
    CSharp,
    Fortran,
    Java,
    Jovial,
    Pascal,
    Plm,
    Python,
    Verilog,
    Vhdl,
    Web,
}

#[derive(Clone)]
pub struct Language {
    name: Lang,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.name {
            Lang::NONE    => write!(f, "{}", "None"),
            Lang::ALL     => write!(f, "{}", "All"),
            Lang::Ada     => write!(f, "{}", "Ada"),
            Lang::Asm     => write!(f, "{}", "Assembler"),
            Lang::Basic   => write!(f, "{}", "Basic"),
            Lang::C       => write!(f, "{}", "C/C++"),
            Lang::Cobol   => write!(f, "{}", "COBOL"),
            Lang::CSharp  => write!(f, "{}", "C#"),
            Lang::Fortran => write!(f, "{}", "FORTRAN"),
            Lang::Java    => write!(f, "{}", "Java"),
            Lang::Jovial  => write!(f, "{}", "JOVIAL"),
            Lang::Pascal  => write!(f, "{}", "Pascal"),
            Lang::Plm     => write!(f, "{}", "PL/M"),
            Lang::Python  => write!(f, "{}", "Python"),
            Lang::Verilog => write!(f, "{}", "Verilog"),
            Lang::Vhdl    => write!(f, "{}", "VHDL"),
            Lang::Web     => write!(f, "{}", "Web"),
        }
    }
}

impl Language {
    pub fn from_raw_language(language: UdbLanguage) -> Option<Language> {
        let lang: u16 = language as u16;
        if lang & UdbLanguage_::Udb_language_Ada as u16 != 0     { return Some(Language{ name: Lang::Ada }) };
        if lang & UdbLanguage_::Udb_language_Asm as u16 != 0     { return Some(Language{ name: Lang::Asm }) };
        if lang & UdbLanguage_::Udb_language_Basic as u16 != 0   { return Some(Language{ name: Lang::Basic }) };
        if lang & UdbLanguage_::Udb_language_C as u16 != 0       { return Some(Language{ name: Lang::C }) };
        if lang & UdbLanguage_::Udb_language_Cobol as u16 != 0   { return Some(Language{ name: Lang::Cobol }) };
        if lang & UdbLanguage_::Udb_language_CSharp as u16 != 0  { return Some(Language{ name: Lang::CSharp }) };
        if lang & UdbLanguage_::Udb_language_Fortran as u16 != 0 { return Some(Language{ name: Lang::Fortran }) };
        if lang & UdbLanguage_::Udb_language_Java as u16 != 0    { return Some(Language{ name: Lang::Java }) };
        if lang & UdbLanguage_::Udb_language_Jovial as u16 != 0  { return Some(Language{ name: Lang::Jovial }) };
        if lang & UdbLanguage_::Udb_language_Pascal as u16 != 0  { return Some(Language{ name: Lang::Pascal }) };
        if lang & UdbLanguage_::Udb_language_Plm as u16 != 0     { return Some(Language{ name: Lang::Plm }) };
        if lang & UdbLanguage_::Udb_language_Python as u16 != 0  { return Some(Language{ name: Lang::Python }) };
        if lang & UdbLanguage_::Udb_language_Verilog as u16 != 0 { return Some(Language{ name: Lang::Verilog }) };
        if lang & UdbLanguage_::Udb_language_Vhdl as u16 != 0    { return Some(Language{ name: Lang::Vhdl }) };
        if lang & UdbLanguage_::Udb_language_Web as u16 != 0     { return Some(Language{ name: Lang::Web }) };
        None
    }
}

pub trait InitLanguage {
    fn init_language(&mut self);
}
