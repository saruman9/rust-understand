use std::os::raw::{c_char, c_int, c_double};

// Opaque structures transform to enumerate
pub enum UdbEntity_ {}
pub enum UdbKindList_ {}
pub enum UdbLexeme_ {}
pub enum UdbLexer_ {}
pub enum UdbLibrary_ {}
pub enum UdbMetric_ {}
pub enum UdbReference_ {}

// Public types

pub type UdbKind      = c_int;
pub type UdbEntity    = *mut UdbEntity_;
pub type UdbKindList  = *mut UdbKindList_;
pub type UdbLexeme    = *mut UdbLexeme_;
pub type UdbLexer     = *mut UdbLexer_;
pub type UdbLibrary   = *mut UdbLibrary_;
pub type UdbMetric    = *mut UdbMetric_;
pub type UdbReference = *mut UdbReference_;

// Public enumerations

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(u8)]
pub enum UdbCommentStyle_ {
    Udb_commentStyleDefault = 0,
    Udb_commentStyleAfter   = 1,
    Udb_commentStyleBefore  = 2
}
pub type UdbCommentStyle = UdbCommentStyle_;

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum UdbCommentFormat_ {
    Udb_commentFormatDefault = 0
}
pub type UdbCommentFormat = UdbCommentFormat_;

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(Copy, Clone)]
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
#[repr(C)]
pub enum UdbMetricKind_ {
    Udb_mkind_NONE = 0,
    Udb_mkind_Integer,
    Udb_mkind_Real
}
pub type UdbMetricKind = UdbMetricKind_;

#[allow(non_camel_case_types)]
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
#[derive(Debug)]
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

pub fn test(){
    println!("");
}

extern {
    // Lookup the comments associated with the specified entity and return a
    // temporary, formatted string.
    pub fn udbComment(entity : UdbEntity,
                      style  : UdbCommentStyle,
                      format : UdbCommentFormat,
                      kinds  : UdbKindList) -> *const c_char;

    // Lookup the comments associated with the specified entity and return a
    // temporary array of raw comment strings.
    pub fn udbCommentRaw(entity        : UdbEntity,
                         style         : UdbCommentStyle,
                         kinds         : UdbKindList,
                         commentString : *mut *const *const c_char,
                         len           : *mut c_int);

    pub fn udbDbClose();

    // Return the major language or languages of the current database.
    pub fn udbDbLanguage() -> UdbLanguage;

    // Return the list of languages required to open a db. No api license is
    // required for this call. Filename is in UTF-8.
    pub fn udbDbLanguages(filename: *const c_char) -> UdbLanguage;

    // Return the filename in UTF-8 of the current database as a temporary string.
    // Name is in absolute, real-case format. Return 0 if no database is open.
    pub fn udbDbName() -> *const c_char;

    // Open a database. Filename is in UTF-8.
    pub fn udbDbOpen(filename: *const c_char) -> UdbStatus;

    // TODO
    // Create a graphic view of the specified entity in a file. The kind of
    // graphic views available are dependent on the language and the kind of
    // entity specified. For example, a function entity may have view kinds of
    // “Invocation”, “Callby”, “Declaration” and “Declaration File”.

    // Graphical view options may optionally be specified as a name and value
    // pair. These options are also dependent on the language and the kind of
    // view specified. For example, the Callby view of a function could
    // optionally show parameters as well as set the font size used, among
    // others.

    pub fn udbEntityDraw(view    : *mut c_char,
                         entity  : UdbEntity,
                         options : *mut c_char,
                         file    : *mut c_char) -> UdbStatus;

    // Return the entity id. This is only valid until the db is changed.
    pub fn udbEntityId(entity: UdbEntity) -> c_int;

    // Return the entity kind.
    pub fn udbEntityKind(entity: UdbEntity) -> UdbKind;

    // Return the entity language.
    pub fn udbEntityLanguage(entity: UdbEntity) -> UdbLanguage;

    // Return the entity library.
    pub fn udbEntityLibrary(entity: UdbEntity) -> UdbLibrary;

    // Return the entity long name as a temporary string. If there is no long name
    // the short name is returned.
    pub fn udbEntityNameLong(entity: UdbEntity) -> *const c_char;

    // Return the absolute name for file entity as a temporary string.
    pub fn udbEntityNameAbsolute(entity: UdbEntity) -> *const c_char;

    // Return the relative name for file entity as a temporary string.
    pub fn udbEntityNameRelative(entity: UdbEntity) -> *const c_char;

    // Return the entity short name as a temporary string.
    pub fn udbEntityNameShort(entity: UdbEntity) -> *const c_char;

    // Return the entity simple name as a temporary string.
    pub fn udbEntityNameSimple(entity: UdbEntity) -> *const c_char;

    // Return the entity unique name as a temporary string.
    pub fn udbEntityNameUnique(entity: UdbEntity) -> *const c_char;

    // Return true if entity can have parameters. If text is specified return a
    // temporary string of parameters prototype. If showname is specified include
    // the names of the parameters.
    pub fn udbEntityParameters(entity    : UdbEntity,
                               text      : *mut *mut c_char,
                               shownames : c_int) -> c_int;

    // Return an allocated list of references, using the refkinds and/or
    // the entkinds specified. Return the length of list. The list of refs
    // must be freed with udbListReferenceFree().
    pub fn udbEntityRefs(entity   : UdbEntity,
                         refkinds : *const c_char,
                         entkinds : *const c_char,
                         unique   : c_int,
                         refs     : *mut *mut UdbReference) -> c_int;

    // Return the entity typetext as a temporary string.
    pub fn udbEntityTypetext(entity: UdbEntity) -> *const c_char;

    // Return a temporary string of the value associated with certain entities
    // such as enumerators, initialized variables, default parameter values in
    // function definitions and macros.
    pub fn udbEntityValue(entity: UdbEntity) -> *const c_char;

    // Return the entity freetext as a temporary string.
    pub fn udbEntityFreetext(entity : UdbEntity,
                             kind   : *const c_char) -> *const c_char;

    // Return the current build.
    pub fn udbInfoBuild() -> *const c_char;

    // Return true if the kind matches the kind text.
    pub fn udbIsKind(kind: UdbKind,
                     text: *const c_char) -> c_int;

    // Return true if the kind refers to a file entity.
    pub fn udbIsKindFile(kind: UdbKind) -> c_int;

    // Return the inverse of the reference kind.
    pub fn udbKindInverse(kind: UdbKind) -> UdbKind;

    // Add a kind to the kindlist if not 0 or allocate a new kindlist.
    pub fn udbKindList(kind     : UdbKind,
                       kindlist : *mut UdbKindList);

    // Return the language of the kind.
    pub fn udbKindLanguage(kind: UdbKind) -> UdbLanguage;

    // Return an allocated copy of kindlist that must be freed with
    // udbKindListFree()
    pub fn udbKindListCopy(kindlist: UdbKindList) -> UdbKindList;

    // Free an allocated kindlist.
    pub fn udbKindListFree(kindlist: UdbKindList);

    // Return true if kind is in the kindlist.
    pub fn udbKindLocate(kind     : UdbKind,
                         kindlist : UdbKindList) -> c_int;

    // Return the long name of kind as a temporary string.
    pub fn udbKindLongname(kind: UdbKind) -> *const c_char;

    // Parse the kind text and return an allocated kindlist that must be freed
    // with udbKindListFree().
    pub fn udbKindParse(text: *const c_char) -> UdbKindList;

    // Return the short name of kind as a temporary string.
    pub fn udbKindShortname(kind: UdbKind) -> *const c_char;

    // Return an array of text representations for a single or multiple language.
    // An entity has a single language but a database may have multiple
    // languagaes. The returned array is 0 terminated and must be freed with
    // udbLanguageStringsFree().
    pub fn udbLanguageStrings(language: UdbLanguage) -> *mut *mut c_char;

    // Free array of languages returned by udbLanguageAsStrings().
    pub fn udbLanguageStringsFree(list: *mut *mut c_char);

    // Return the lexeme beginning column.
    pub fn udbLexemeColumnBegin(lexeme: UdbLexeme) -> c_int;

    // Return the lexeme ending column.
    pub fn udbLexemeColumnEnd(lexeme: UdbLexeme) -> c_int;

    // Return the lexeme associated entity.
    pub fn udbLexemeEntity(lexeme: UdbLexeme) -> UdbEntity;

    // Return true if the lexeme is part of inactive code.
    pub fn udbLexemeInactive(lexeme: UdbLexeme) -> c_int;

    // Return the lexeme beginning line.
    pub fn udbLexemeLineBegin(lexeme: UdbLexeme) -> c_int;

    // Return the lexeme ending line.
    pub fn udbLexemeLineEnd(lexeme: UdbLexeme) -> c_int;

    // Return the next lexeme, or 0 for end.
    pub fn udbLexemeNext(lexeme: UdbLexeme) -> UdbLexeme;

    // Return the previous lexeme, or 0 for beginning.
    pub fn udbLexemePrevious(lexeme: UdbLexeme) -> UdbLexeme;

    // Return the lexeme associated reference, or 0.
    pub fn udbLexemeReference(lexeme: UdbLexeme) -> UdbReference;

    // Return the lexeme text as a temporary string.
    pub fn udbLexemeText(lexeme: UdbLexeme) -> *const c_char;

    // Return the lexeme token.
    pub fn udbLexemeToken(lexeme: UdbLexeme) -> UdbToken;

    // Return the name of a lexeme token.
    pub fn udbLexemeTokenName(token: UdbToken) -> *const c_char;

    // Delete a lexer and all its lexemes.
    pub fn udbLexerDelete(lexer: UdbLexer);

    // Return the first lexeme of the lexer.
    pub fn udbLexerFirst(lexer: UdbLexer) -> UdbLexeme;

    // Return the lexeme that occurs at the specified line/column.
    pub fn udbLexerLexeme(lexer : UdbLexer,
                          line  : c_int,
                          col   : c_int) -> UdbLexeme;

    // Return a temporary array of lexemes within the specified range of lines.
    // If no lexeme begins on the starting line, the lexeme that includes the
    // starting line will be returned. The returned list is temporary and will be
    // made invalid on the next call to this function. Return size of array.
    pub fn udbLexerLexemes(lexer     : UdbLexer,
                           startLine : c_int,
                           endLine   : c_int,
                           lexeme    : *mut *mut UdbLexeme) -> c_int;

    // Return the number of lines in the specified lexer.
    pub fn udbLexerLines(lexer: UdbLexer) -> c_int;

    // Create a lexer for the file entity. The physical source file associated
    // with the entity must be readable and must not be changed since the database
    // was last updated.
    pub fn udbLexerNew(entity : UdbEntity,
                       ents   : c_int,
                       lexer  : *mut UdbLexer) -> UdbStatus;

    // Return true if the specified entity is in the specified list of libraries.
    pub fn udbLibraryCheckEntity(entity  : UdbEntity,
                                 liblist : *mut UdbLibrary) -> c_int;

    // Compare 2 library structures. Return a value suitable for typical
    // compare operations (ie, 0=same, <0=lib1<lib2, >0=lib1>lib2).
    pub fn udbLibraryCompare(lib1: UdbLibrary,
                             lib2: UdbLibrary) -> c_int;

    // Filter the specified list of entities with the specified library
    // filter and return a new, allocated list.
    pub fn udbLibraryFilterEntity(ents    : *mut UdbEntity,
                                  filter  : *const c_char,
                                  newents : *mut *mut UdbEntity,
                                  num     : *mut c_int);

    // Return an allocated list of libraries. Use udbLibraryListFree() to
    // free the list.
    pub fn udbLibraryList(filter  : *const c_char,
                          liblist : *mut *mut UdbLibrary,
                          num     : *mut c_int);

    // Free the allocated library list.
    pub fn udbLibraryListFree(liblist: *mut UdbLibrary);

    // Return the library name as a temporary string.
    pub fn udbLibraryName(library: UdbLibrary) -> *const c_char;

    // Return a non-allocated, permanent list of all entities. After a database
    // update, the list is invalid and must be retrieved again. This list may be
    // used in places where an allocated entity list is required and may be
    // safely passed to udbListEntityFree().
    pub fn udbListEntity(list  : *mut *mut UdbEntity,
                         items : *mut c_int);

    // Filter the specified list of entities, using the kinds specified, and return
    // a new allocated array. Use udbListEntityFree() to free this list. The
    // original list of entities and the kindlist must both be allocated and will
    // be freed by this call.
    pub fn udbListEntityFilter(ents    : *mut UdbEntity,
                               kinds   : UdbKindList,
                               newents : *mut *mut UdbEntity,
                               items   : *mut c_int);

    // Free an allocated list of entities.
    pub fn udbListEntityFree(list: *mut UdbEntity);

    // Return a temporary list of all analyzed file entities.
    pub fn udbListFile(list  : *mut *mut UdbEntity,
                       items : *mut c_int);

    // Return allocated list of all entity kinds. Call udbListKindFree() to free
    // list.
    pub fn udbListKindEntity(list  : *mut *mut UdbKind,
                             items : *mut c_int);

    // Free an allocated list of kinds.
    pub fn udbListKindFree(list: *mut UdbKind);

    // Return allocated list of all reference kinds. Call udbListKindFree() to
    // free list.
    pub fn udbListKindReference(list  : *mut *mut UdbKind,
                                items : *mut c_int);

    // Return an allocated list of all references for entity.
    // Free the list with udbListReferenceFree().
    pub fn udbListReference(entity : UdbEntity,
                            refs   : *mut *mut UdbReference,
                            items  : *mut c_int);

    // Return an allocated list of all references within file.
    // Free the list with udbListReferenceFree().
    pub fn udbListReferenceFile(file  : UdbEntity,
                                refs  : *mut *mut UdbReference,
                                items : *mut c_int);

    // Filter the specified list of references, using the refkinds and/or the
    // entkinds specified, and return a new allocated array. If unique is
    // specified, the newrefs array will only contain the first reference for
    // each unique entity. Refkinds and Entkinds must both be allocated and
    // will be freed by this call.
    pub fn udbListReferenceFilter(refs     : *mut UdbReference,
                                  refkinds : UdbKindList,
                                  entkinds : UdbKindList,
                                  unique   : c_int,
                                  refs     : *mut *mut UdbReference,
                                  num      : *mut c_int);

    // Free the allocated references list.
    pub fn udbListReferenceFree(refs: *mut UdbReference);

    // Lookup and return an allocated list of entities by name and kind, if
    // specified.
    pub fn udbLookupEntity(name      : *const c_char,
                           kind      : *const c_char,
                           shortname : c_int,
                           ents      : *mut *mut UdbEntity,
                           num       : *mut c_int);

    // Lookup an entity by name and file, line and column where it is referenced.
    pub fn udbLookupEntityByReference(file      : UdbEntity,
                                      name      : *const c_char,
                                      line      : c_int,
                                      col       : c_int,
                                      matchline : *mut c_int) -> UdbEntity;

    // Lookup an entity by unique name.
    pub fn udbLookupEntityByUniquename(uniquename: *const c_char) -> UdbEntity;

    // Lookup a project file by long or short name.
    pub fn udbLookupFile(name: *const c_char) -> UdbEntity;

    // Lookup the expansion text for a macro name at a location identified by file,
    // line and column. Return true if found, or false if not. Return a temporary
    // copy of the expansion text. This is only available for C++ files with the
    // option "Save macro expansion text" enabled.
    pub fn udbLookupMacroExpansionText(name   : *const c_char,
                                       file   : UdbEntity,
                                       line   : c_int,
                                       column : c_int,
                                       text   : *mut *mut c_char) -> c_int;

    // Return true if the specified entity has any reference of the general kind
    // specified by the list of references. Return true if the list is 0. Kindlist
    // must be allocated and will be deleted.
    pub fn udbLookupReferenceExists(entity   : UdbEntity,
                                    kindlist : UdbKindList) -> c_int;

    // Return the description of the specified metric, as a temporary string.
    pub fn udbMetricDescription(metric: UdbMetric) -> *const c_char;

    // Return the descriptive name of the specified metric, as a temporary string.
    pub fn udbMetricDescriptiveName(metric: UdbMetric) -> *const c_char;

    // Return true if the specified metric is defined for the specified entity.
    pub fn udbMetricIsDefinedEntity(metric: UdbMetric,
                                    entity: UdbEntity) -> c_int;

    // Return true if the specified metric is defined as a project metric for the
    // specified language.
    pub fn udbMetricIsDefinedProject(metric   : UdbMetric,
                                     language : UdbLanguage) -> c_int;

    // Return the value kind of a metric.
    pub fn udbMetricKind(metric: UdbMetric) -> UdbMetricKind;

    // Return the size of a temporary, null-terminated list of all metrics defined
    // for the specified entity.
    pub fn udbMetricListEntity(entity  : UdbEntity,
                               metrics : *mut *mut UdbMetric) -> c_int;

    // Return the size of a temporary, null-terminated list of all metrics defined
    // for the specified ent kinds filter.
    pub fn udbMetricListKind(name    : *const c_char,
                             metrics : *mut *mut UdbMetric) -> c_int;

    // Return the size of a temporary, null-terminated list of all metrics defined
    // for the specified language.
    pub fn udbMetricListLanguage(language : UdbLanguage,
                                 metrics  : *mut *mut UdbMetric) -> c_int;

    // Return the size of a temporary, null-terminated list of all project metrics
    // defined for the specified language.
    pub fn udbMetricListProject(language : UdbLanguage,
                                metrics  : *mut *mut UdbMetric) -> c_int;

    // Lookup a metric by name.
    pub fn udbMetricLookup(name: *const c_char) -> UdbMetric;

    // Return the name of the specified metric, as a temporary string.
    pub fn udbMetricName(metric: UdbMetric) -> *mut c_char;

    // Return the value of a metric for the specified entity.
    pub fn udbMetricValue(entity: UdbEntity,
                          metric: UdbMetric) -> c_double;

    // Return the value of a project metric.
    pub fn udbMetricValueProject(metric: UdbMetric) -> c_double;

    // Return reference column.
    pub fn udbReferenceColumn(reference: UdbReference) -> c_int;

    // Return an allocated copy of reference.
    pub fn udbReferenceCopy(reference: UdbReference) -> UdbReference;

    // Free reference copied by udbReferenceCopy().
    pub fn udbReferenceCopyFree(reference: UdbReference);

    // Return reference entity.
    pub fn udbReferenceEntity(reference: UdbReference) -> UdbEntity;

    // Return reference file.
    pub fn udbReferenceFile(reference: UdbReference) -> UdbEntity;

    // Return reference kind.
    pub fn udbReferenceKind(reference: UdbReference) -> UdbKind;

    // Return reference line.
    pub fn udbReferenceLine(reference: UdbReference) -> c_int;

    // Return reference scope.
    pub fn udbReferenceScope(reference: UdbReference) -> UdbEntity;

    pub fn udbSetLicense(dir: *const c_char);

    pub fn udbStatusText(status: UdbStatus) -> *const c_char;
}

// TODO delete tests, because unsafe
#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};

    use super::{UdbStatus_, udbInfoBuild, udbDbOpen};

    #[test]
    fn get_info_build() {
        assert_eq!("851", unsafe { CStr::from_ptr(udbInfoBuild()).to_string_lossy() });
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
