extern crate understand_sys;

use std::fmt;

use understand_sys::{UdbLanguage_, UdbLanguage};


#[derive(Clone)]
pub enum Language {
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

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Language::NONE    => write!(f, "{}", "None"),
            Language::ALL     => write!(f, "{}", "All"),
            Language::Ada     => write!(f, "{}", "Ada"),
            Language::Asm     => write!(f, "{}", "Assembler"),
            Language::Basic   => write!(f, "{}", "Basic"),
            Language::C       => write!(f, "{}", "C/C++"),
            Language::Cobol   => write!(f, "{}", "COBOL"),
            Language::CSharp  => write!(f, "{}", "C#"),
            Language::Fortran => write!(f, "{}", "FORTRAN"),
            Language::Java    => write!(f, "{}", "Java"),
            Language::Jovial  => write!(f, "{}", "JOVIAL"),
            Language::Pascal  => write!(f, "{}", "Pascal"),
            Language::Plm     => write!(f, "{}", "PL/M"),
            Language::Python  => write!(f, "{}", "Python"),
            Language::Verilog => write!(f, "{}", "Verilog"),
            Language::Vhdl    => write!(f, "{}", "VHDL"),
            Language::Web     => write!(f, "{}", "Web"),
        }
    }
}

impl Language {
    pub fn from_raw(language: UdbLanguage) -> Option<Language> {
        let lang: u16 = language as u16;
        if lang & UdbLanguage_::Udb_language_Ada as u16 != 0     { return Some(Language::Ada) };
        if lang & UdbLanguage_::Udb_language_Asm as u16 != 0     { return Some(Language::Asm) };
        if lang & UdbLanguage_::Udb_language_Basic as u16 != 0   { return Some(Language::Basic) };
        if lang & UdbLanguage_::Udb_language_C as u16 != 0       { return Some(Language::C) };
        if lang & UdbLanguage_::Udb_language_Cobol as u16 != 0   { return Some(Language::Cobol) };
        if lang & UdbLanguage_::Udb_language_CSharp as u16 != 0  { return Some(Language::CSharp) };
        if lang & UdbLanguage_::Udb_language_Fortran as u16 != 0 { return Some(Language::Fortran) };
        if lang & UdbLanguage_::Udb_language_Java as u16 != 0    { return Some(Language::Java) };
        if lang & UdbLanguage_::Udb_language_Jovial as u16 != 0  { return Some(Language::Jovial) };
        if lang & UdbLanguage_::Udb_language_Pascal as u16 != 0  { return Some(Language::Pascal) };
        if lang & UdbLanguage_::Udb_language_Plm as u16 != 0     { return Some(Language::Plm) };
        if lang & UdbLanguage_::Udb_language_Python as u16 != 0  { return Some(Language::Python) };
        if lang & UdbLanguage_::Udb_language_Verilog as u16 != 0 { return Some(Language::Verilog) };
        if lang & UdbLanguage_::Udb_language_Vhdl as u16 != 0    { return Some(Language::Vhdl) };
        if lang & UdbLanguage_::Udb_language_Web as u16 != 0     { return Some(Language::Web) };
        None
    }
}
