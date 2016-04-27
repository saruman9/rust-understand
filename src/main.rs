use std::os::raw::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::mem;

#[allow(dead_code)]
#[repr(C)]
struct UdbEntity_;
#[allow(dead_code)]
#[repr(C)]
struct UdbKindList_;
#[allow(dead_code)]
#[repr(C)]
struct UdbLexeme_;
#[allow(dead_code)]
#[repr(C)]
struct UdbLexer_;
#[allow(dead_code)]
#[repr(C)]
struct UdbLibrary_;
#[allow(dead_code)]
#[repr(C)]
struct UdbMetric_;
#[allow(dead_code)]
#[repr(C)]
struct UdbReference_;

type UdbEntity    = *mut c_void;
type UdbKindList  = *mut UdbKindList_;
type UdbLexeme    = *mut UdbLexeme_;
type UdbLexer     = *mut UdbLexer_;
type UdbLibrary   = *mut UdbLibrary_;
type UdbMetric    = *mut UdbMetric_;
type UdbReference = *mut UdbReference_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
enum UdbCommentStyle_ {
    Udb_commentStyleDefault = 0,
    Udb_commentStyleAfter   = 1,
    Udb_commentStyleBefore  = 2
}
type UdbCommentStyle = UdbCommentStyle_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
enum UdbCommentFormat_ {
    Udb_commentFormatDefault = 0
}
type UdbCommentFormat = UdbCommentStyle_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u16)]
enum UdbLanguage_ {
    Udb_language_NONE    = 0x0000,
    Udb_language_ALL     = 0x7FFF,
    Udb_language_Ada     = 0x0001,
    Udb_language_Asm     = 0x0002,
    Udb_language_Basic   = 0x0004,
    Udb_language_C       = 0x0008,
    Udb_language_Cobol   = 0x0010,
    Udb_language_CSharp  = 0x0020,
    Udb_language_Fortran = 0x0040,
    Udb_language_Java    = 0x0080,
    Udb_language_Jovial  = 0x0100,
    Udb_language_Pascal  = 0x0200,
    Udb_language_Plm     = 0x0400,
    Udb_language_Python  = 0x0800,
    Udb_language_Verilog = 0x1000,
    Udb_language_Vhdl    = 0x2000,
    Udb_language_Web     = 0x4000,
}
type UdbLanguage = UdbLanguage_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(i64)]
enum UdbMetricKind_ {
    Udb_mkind_NONE=0,
    Udb_mkind_Integer,
    Udb_mkind_Real
}
type UdbMetricKind = UdbMetricKind_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug)]
enum UdbStatus_ {
    Udb_statusOkay                     = 0,
    Udb_statusDBAlreadyOpen            = 1,
    Udb_statusDBBusy                   = 2, /* not used */
    Udb_statusDBChanged                = 3,
    Udb_statusDBCorrupt                = 4,
    Udb_statusDBOldVersion             = 5,
    Udb_statusDBUnknownVersion         = 6,
    Udb_statusDBUnableCreate           = 7,
    Udb_statusDBUnableDelete           = 8,
    Udb_statusDBUnableModify           = 9,
    Udb_statusDBUnableOpen             = 10,
    Udb_statusDBUnableWrite            = 11,
    Udb_statusDemoAnotherDBOpen        = 12,
    Udb_statusDemoInvalid              = 13,
    Udb_statusDrawNoFont               = 14,
    Udb_statusDrawNoImage              = 15,
    Udb_statusDrawTooBig               = 16,
    Udb_statusDrawUnableCreateFile     = 17,
    Udb_statusDrawUnsupportedFile      = 18,
    Udb_statusLexerFileModified        = 19,
    Udb_statusLexerFileUnreadable      = 20,
    Udb_statusLexerUnsupportedLanguage = 21,
    Udb_statusNoApiLicense             = 22,
    Udb_statusNoApiLicenseAda          = 23,
    Udb_statusNoApiLicenseC            = 24,
    Udb_statusNoApiLicenseCobol        = 25,
    Udb_statusNoApiLicenseFtn          = 26,
    Udb_statusNoApiLicenseJava         = 27,
    Udb_statusNoApiLicenseJovial       = 28,
    Udb_statusNoApiLicensePascal       = 29,
    Udb_statusNoApiLicensePlm          = 30,
    Udb_statusNoApiLicensePython       = 31,
    Udb_statusNoApiLicenseWeb          = 32,
    Udb_statusNoApiLicenseVhdl         = 33,
    Udb_statusNoApiLicenseVerilog      = 34,
    Udb_statusReportUnableCreate       = 35,
    Udb_statusReportUnableDelete       = 36,
    Udb_statusReportUnableWrite        = 37,
    Udb_statusUserAbort                = 38,
    Udb_statusWrongProduct             = 39,
    Udb_status_LAST
}
type UdbStatus = UdbStatus_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
enum UdbToken_ {
    Udb_tokenEOF            = 0,
    Udb_tokenComment        = 1,
    Udb_tokenContinuation   = 2,
    Udb_tokenDedent         = 3,
    Udb_tokenEndOfStatement = 4,
    Udb_tokenIdentifier     = 5,
    Udb_tokenIndent         = 6,
    Udb_tokenKeyword        = 7,
    Udb_tokenLabel          = 8,
    Udb_tokenLiteral        = 9,
    Udb_tokenNewline        = 10,
    Udb_tokenOperator       = 11,
    Udb_tokenPreprocessor   = 12,
    Udb_tokenPunctuation    = 13,
    Udb_tokenString         = 14,
    Udb_tokenWhitespace     = 15,
    Udb_token_LAST
}
type UdbToken = UdbToken_;

extern {
    fn udbInfoBuild() -> *const c_char;

    fn udbDbClose();

    // Open a database. Filename is in UTF-8.
    fn udbDbOpen(filename: *const c_char) -> UdbStatus;

    // Return the entity long name as a temporary string. If there is no long name
    // the short name is returned.
    fn udbEntityNameLong(udb_entity: UdbEntity) -> *mut c_char;

    // Return a non-allocated, permanent list of all entities. After a database
    // update, the list is invalid and must be retrieved again. Ths list may be
    // used in places where an allocated entity list is required and may be
    // safely passed to udbListEntityFree().
    fn udbListEntity(udb_list_entity: *mut UdbEntity, ents_size: *mut c_int);

    // Free an allocated list of entities.
    fn udbListEntityFree(udb_entity: UdbEntity);
}

fn main() {
    let x = unsafe { CStr::from_ptr(udbInfoBuild()).to_string_lossy() };
    println!("Build: {}", x);

    let db_filename = CString::new("./test.udb").unwrap().as_ptr();
    let status: UdbStatus = unsafe { udbDbOpen(db_filename) };
    let mut ents: UdbEntity = unsafe { mem::uninitialized() };
    let mut ents_size: c_int = 0;
    let i: i32 = 0;
    unsafe { udbListEntity(&mut ents, &mut ents_size) };
    println!("{:?}", status);
    println!("{}", ents_size);

    /*
    for x in 0..ents_size {
        println!("{:?}", udbEntityNameLong(ents[x]));
    }
     */


    unsafe { udbListEntityFree(ents) };
    unsafe { udbDbClose() };
}
