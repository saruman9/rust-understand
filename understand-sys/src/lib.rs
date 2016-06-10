use std::os::raw::{c_char, c_int};

// Opaque structs transform to enum
pub enum UdbEntity_ {}
#[allow(dead_code)]
pub enum UdbKindList_ {}
#[allow(dead_code)]
pub enum UdbLexeme_ {}
#[allow(dead_code)]
pub enum UdbLexer_ {}
#[allow(dead_code)]
pub enum UdbLibrary_ {}
#[allow(dead_code)]
pub enum UdbMetric_ {}
#[allow(dead_code)]
pub enum UdbReference_ {}

pub type UdbKind      = c_int;
pub type UdbEntity    = *mut UdbEntity_;
pub type UdbKindList  = *mut UdbKindList_;
pub type UdbLexeme    = *mut UdbLexeme_;
pub type UdbLexer     = *mut UdbLexer_;
pub type UdbLibrary   = *mut UdbLibrary_;
pub type UdbMetric    = *mut UdbMetric_;
pub type UdbReference = *mut UdbReference_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
pub enum UdbCommentStyle_ {
    Udb_commentStyleDefault = 0,
    Udb_commentStyleAfter   = 1,
    Udb_commentStyleBefore  = 2
}
pub type UdbCommentStyle = UdbCommentStyle_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
pub enum UdbCommentFormat_ {
    Udb_commentFormatDefault = 0
}
pub type UdbCommentFormat = UdbCommentFormat_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u16)]
pub enum UdbLanguage_ {
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
pub type UdbLanguage = UdbLanguage_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(C)]
pub enum UdbMetricKind_ {
    Udb_mkind_NONE = 0,
    Udb_mkind_Integer,
    Udb_mkind_Real
}
pub type UdbMetricKind = UdbMetricKind_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug)]
pub enum UdbStatus_ {
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
pub type UdbStatus = UdbStatus_;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
pub enum UdbToken_ {
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
pub type UdbToken = UdbToken_;

extern {
    // Return the current build.
    pub fn udbInfoBuild() -> *const c_char;

    // Close a database.
    pub fn udbDbClose();

    // Open a database. Filename is in UTF-8.
    pub fn udbDbOpen(filename: *const c_char) -> UdbStatus;

    // Return the entity long name as a temporary string. If there is no long name
    // the short name is returned.
    pub fn udbEntityNameLong(udb_entity: UdbEntity) -> *const c_char;

    // Return allocated list of all entity kinds. Call udbListKindFree() to free
    // list.
    pub fn udbListKindEntity(udb_kind_list: *const *const UdbKind,
                         kinds_size: *const c_int);

    // Parse the kind text and return an allocated kindlist that must be freed
    // with udbkindlistfree().
    pub fn udbKindParse(kind_text: *const c_char) -> UdbKindList;

    // Return the entity kind.
    pub fn udbEntityKind(udb_entity: UdbEntity) -> UdbKind;

    // Free an allocated kindlist.
    pub fn udbKindListFree(udb_kind_list: UdbKindList);

    // Return the short name of kind as a temporary string.
    pub fn udbKindShortname(udb_kind: UdbKind) -> *mut c_char;

    // Return the long name of kind as a temporary string.
    pub fn udbKindLongname(udb_kind: UdbKind) -> *mut c_char;

    // Return a non-allocated, permanent list of all entities. After a database
    // update, the list is invalid and must be retrieved again. Ths list may be
    // used in places where an allocated entity list is required and may be
    // safely passed to udbListEntityFree().
    pub fn udbListEntity(udb_entity_list: *mut *mut UdbEntity, ents_size: *mut c_int);

    // Filter the specified list of entities, using the kinds specified, and return
    // a new allocated array. Use udbListEntityFree() to free this list. The
    // original list of entities and the kindlist must both be allocated and will
    // be freed by this call.
    pub fn udbListEntityFilter(udb_entity      : *mut UdbEntity,
                               udb_list_kind   : UdbKindList,
                               udb_entity_list : *mut *mut UdbEntity,
                               ents_size       : *mut c_int);

    // Free an allocated list of entities.
    pub fn udbListEntityFree(udb_entity: *const UdbEntity);
}

// TODO delete tests, because unsafe
#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};

    use super::{UdbStatus_, udbInfoBuild, udbDbOpen};

    #[test]
    fn get_info_build() {
        assert_eq!("833", unsafe { CStr::from_ptr(udbInfoBuild()).to_string_lossy() });
    }

    #[test]
    fn get_status_open() {
        let udb_db_path: CString = CString::new("test.udb").unwrap();

        match unsafe { udbDbOpen(udb_db_path.as_ptr()) } {
            UdbStatus_::Udb_statusOkay => return,
            _ => panic!("Unexpected status of udb DB")
        }
    }
}
