extern crate understand_sys;

use std::fmt;

use understand_sys::{UdbLanguage_, UdbLanguage};


#[derive(PartialEq, Debug)]
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

impl Language {
    pub fn from_raw(raw: UdbLanguage) -> Option<Language> {
        let lang: u16 = raw as u16;
        if lang & UdbLanguage_::Udb_language_Ada as u16 != 0 {
            return Some(Language::Ada);
        };
        if lang & UdbLanguage_::Udb_language_Asm as u16 != 0 {
            return Some(Language::Asm);
        };
        if lang & UdbLanguage_::Udb_language_Basic as u16 != 0 {
            return Some(Language::Basic);
        };
        if lang & UdbLanguage_::Udb_language_C as u16 != 0 {
            return Some(Language::C);
        };
        if lang & UdbLanguage_::Udb_language_Cobol as u16 != 0 {
            return Some(Language::Cobol);
        };
        if lang & UdbLanguage_::Udb_language_CSharp as u16 != 0 {
            return Some(Language::CSharp);
        };
        if lang & UdbLanguage_::Udb_language_Fortran as u16 != 0 {
            return Some(Language::Fortran);
        };
        if lang & UdbLanguage_::Udb_language_Java as u16 != 0 {
            return Some(Language::Java);
        };
        if lang & UdbLanguage_::Udb_language_Jovial as u16 != 0 {
            return Some(Language::Jovial);
        };
        if lang & UdbLanguage_::Udb_language_Pascal as u16 != 0 {
            return Some(Language::Pascal);
        };
        if lang & UdbLanguage_::Udb_language_Plm as u16 != 0 {
            return Some(Language::Plm);
        };
        if lang & UdbLanguage_::Udb_language_Python as u16 != 0 {
            return Some(Language::Python);
        };
        if lang & UdbLanguage_::Udb_language_Verilog as u16 != 0 {
            return Some(Language::Verilog);
        };
        if lang & UdbLanguage_::Udb_language_Vhdl as u16 != 0 {
            return Some(Language::Vhdl);
        };
        if lang & UdbLanguage_::Udb_language_Web as u16 != 0 {
            return Some(Language::Web);
        };
        None
    }

    /// Convert `Language` to string.
    pub fn to_string(&self) -> String {
        match *self {
            Language::NONE => "None".to_string(),
            Language::ALL => "All".to_string(),
            Language::Ada => "Ada".to_string(),
            Language::Asm => "Assembler".to_string(),
            Language::Basic => "Basic".to_string(),
            Language::C => "C/C++".to_string(),
            Language::Cobol => "COBOL".to_string(),
            Language::CSharp => "C#".to_string(),
            Language::Fortran => "FORTRAN".to_string(),
            Language::Java => "Java".to_string(),
            Language::Jovial => "JOVIAL".to_string(),
            Language::Pascal => "Pascal".to_string(),
            Language::Plm => "PL/M".to_string(),
            Language::Python => "Python".to_string(),
            Language::Verilog => "Verilog".to_string(),
            Language::Vhdl => "VHDL".to_string(),
            Language::Web => "Web".to_string(),
        }
    }
}

impl<S: AsRef<str>> From<S> for Language {
    fn from(s: S) -> Self {
        match s.as_ref() {
            "All" => Language::ALL,
            "Ada" => Language::Ada,
            "Assembler" => Language::Asm,
            "Basic" => Language::Basic,
            "C/C++" => Language::C,
            "COBOL" => Language::Cobol,
            "C#" => Language::CSharp,
            "FORTRAN" => Language::Fortran,
            "Java" => Language::Java,
            "JOVIAL" => Language::Jovial,
            "Pascal" => Language::Pascal,
            "PL/M" => Language::Plm,
            "Python" => Language::Python,
            "Verilog" => Language::Verilog,
            "VHDL" => Language::Vhdl,
            "Web" => Language::Web,
            _ => Language::NONE,
        }
    }
}


impl Default for Language {
    fn default() -> Language {
        Language::NONE
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
