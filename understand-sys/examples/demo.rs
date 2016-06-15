extern crate understand_sys;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char};
use std::env::args;
use std::mem;

use understand_sys::*;

fn main() {
    let program_args: Vec<String> = args().collect();
    if program_args.len() != 2 {
        println!("Expected one argument - path to UDB database.");
        std::process::exit(2);
    }

    // Get number of build
    unsafe{
        let num_build: &str = CStr::from_ptr(udbInfoBuild()).to_str().unwrap();
        println!("num_build: {}", num_build);
    }

    unsafe {
        // Open the UDB database
        let udb_path_str: *mut c_char = CString::new(&program_args[1][..]).unwrap().into_raw();
        let udb_status: UdbStatus = udbDbOpen(udb_path_str);
        println!("udb_status: {:?}", udb_status);

        // Get status of UDB database
        let udb_status_text: &CStr = CStr::from_ptr(udbStatusText(udb_status));
        println!("udb_status_text: {}", udb_status_text.to_string_lossy());
    }

    unsafe{
        // Get bitmask of languages in UDB database
        let udb_languages: UdbLanguage = udbDbLanguage();
        let udb_languages_vec: Vec<&'static str> = get_language_from_bitmask(udb_languages);
        println!("udb_languages:");
        for language in udb_languages_vec {
            println!("\t{}", language);
        }

        // Get first string from ptr to structure of languages
        let udb_language_strings = udbLanguageStrings(udb_languages);
        let lang: &CStr = CStr::from_ptr(*udb_language_strings.offset(0));
        println!("{:?}", lang);
    }

    let udb_language_strings = unsafe { udbLanguageStrings(udb_languages) };
    println!("{:?}", udb_language_strings);

    /*
    let udb_languages_strings: Vec<&CStr> = unsafe {
        let vec: Vec<&CStr> = vec![];
        for i in 
        // CStr::from_ptr(udbLanguageStrings(udb_languages))
        vec
    };
    */

    unsafe { udbDbClose() };
}

fn get_language_from_bitmask(language: UdbLanguage) -> Vec<&'static str> {
    let language = language as u16;
    let mut vec_of_languages: Vec<&'static str> = Vec::new();
    if (language & (UdbLanguage_::Udb_language_Ada as u16)) != 0 {
        vec_of_languages.push("Ada");
    }
    if (language & (UdbLanguage_::Udb_language_Asm as u16)) != 0 {
        vec_of_languages.push("Asm");
    }
    if (language & (UdbLanguage_::Udb_language_Basic as u16)) != 0 {
        vec_of_languages.push("Basic");
    }
    if (language & (UdbLanguage_::Udb_language_C as u16)) != 0 {
        vec_of_languages.push("C");
    }
    if (language & (UdbLanguage_::Udb_language_CSharp as u16)) != 0 {
        vec_of_languages.push("C#");
    }
    if (language & (UdbLanguage_::Udb_language_Cobol as u16)) != 0 {
        vec_of_languages.push("COBOL");
    }
    if (language & (UdbLanguage_::Udb_language_Fortran as u16)) != 0 {
        vec_of_languages.push("FORTRAN");
    }
    if (language & (UdbLanguage_::Udb_language_Java as u16)) != 0 {
        vec_of_languages.push("Java");
    }
    if ((language as u16) & (UdbLanguage_::Udb_language_Jovial as u16)) != 0 {
        vec_of_languages.push("Jovial");
    }
    if (language & (UdbLanguage_::Udb_language_Pascal as u16)) != 0 {
        vec_of_languages.push("Pascal");
    }
    if (language & (UdbLanguage_::Udb_language_Plm as u16)) != 0 {
        vec_of_languages.push("PL/M");
    }
    if (language & (UdbLanguage_::Udb_language_Python as u16)) != 0 {
        vec_of_languages.push("Python");
    }
    if (language & (UdbLanguage_::Udb_language_Verilog as u16)) != 0 {
        vec_of_languages.push("Verilog");
    }
    if (language & (UdbLanguage_::Udb_language_Vhdl as u16)) != 0 {
        vec_of_languages.push("VHDL");
    }
    if (language & (UdbLanguage_::Udb_language_Web as u16)) != 0 {
        vec_of_languages.push("WEB");
    }
    vec_of_languages
}
