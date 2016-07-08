use std::fmt;

// TODO udbStatusText(status: UdbStatus) -> *const c_char - ?
pub enum StatusError {
    Okay,
    DBAlreadyOpen,
    DBBusy,
    DBChanged,
    DBCorrupt,
    DBOldVersion,
    DBUnknownVersion,
    DBUnableCreate,
    DBUnableDelete,
    DBUnableModify,
    DBUnableOpen,
    DBUnableWrite,
    DemoAnotherDBOpen,
    DemoInvalid,
    DrawNoFont,
    DrawNoImage,
    DrawTooBig,
    DrawUnableCreateFile,
    DrawUnsupportedFile,
    LexerFileModified,
    LexerFileUnreadable,
    LexerUnsupportedLanguage,
    NoApiLicense,
    NoApiLicenseAda,
    NoApiLicenseC,
    NoApiLicenseCobol,
    NoApiLicenseFtn,
    NoApiLicenseJava,
    NoApiLicenseJovial,
    NoApiLicensePascal,
    NoApiLicensePlm,
    NoApiLicensePython,
    NoApiLicenseWeb,
    NoApiLicenseVhdl,
    NoApiLicenseVerilog,
    ReportUnableCreate,
    ReportUnableDelete,
    ReportUnableWrite,
    UserAbort,
    WrongProduct,
}
impl StatusError {
    pub fn __description(&self) -> &str {
        match *self {
            StatusError::Okay                     => "okay",
            StatusError::DBAlreadyOpen            => "database already open",
            StatusError::DBBusy                   => "database busy",
            StatusError::DBChanged                => "database changed",
            StatusError::DBCorrupt                => "database corrupt",
            StatusError::DBOldVersion             => "old version of database",
            StatusError::DBUnknownVersion         => "unknown version of database",
            StatusError::DBUnableCreate           => "unable to create database",
            StatusError::DBUnableDelete           => "unable to delete database",
            StatusError::DBUnableModify           => "unable to modify database",
            StatusError::DBUnableOpen             => "unable to open database",
            StatusError::DBUnableWrite            => "unable to write database",
            StatusError::DemoAnotherDBOpen        => "Demo: another database open",
            StatusError::DemoInvalid              => "Demo: invalid",
            StatusError::DrawNoFont               => "Draw: no font",
            StatusError::DrawNoImage              => "Draw: no image",
            StatusError::DrawTooBig               => "Draw: too big for draw",
            StatusError::DrawUnableCreateFile     => "Draw: unable to create file",
            StatusError::DrawUnsupportedFile      => "Draw: unsupported file",
            StatusError::LexerFileModified        => "Lexer: file modified",
            StatusError::LexerFileUnreadable      => "Lexer: file unreadable",
            StatusError::LexerUnsupportedLanguage => "Lexer: unsupported language",
            StatusError::NoApiLicense             => "API: no license",
            StatusError::NoApiLicenseAda          => "API: no license Ada",
            StatusError::NoApiLicenseC            => "API: no license C",
            StatusError::NoApiLicenseCobol        => "API: no license COBOL",
            StatusError::NoApiLicenseFtn          => "API: no license FORTRAN",
            StatusError::NoApiLicenseJava         => "API: no license Java",
            StatusError::NoApiLicenseJovial       => "API: no license Jovial",
            StatusError::NoApiLicensePascal       => "API: no license Pascal",
            StatusError::NoApiLicensePlm          => "API: no license PL/M",
            StatusError::NoApiLicensePython       => "API: no license Python",
            StatusError::NoApiLicenseWeb          => "API: no license web",
            StatusError::NoApiLicenseVhdl         => "API: no license VHDL",
            StatusError::NoApiLicenseVerilog      => "API: no license Verilog",
            StatusError::ReportUnableCreate       => "Report: unable create",
            StatusError::ReportUnableDelete       => "Report: unable delete",
            StatusError::ReportUnableWrite        => "Report: unable write",
            StatusError::UserAbort                => "abort by user",
            StatusError::WrongProduct             => "wrong product",
        }
    }
}
impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.__description().fmt(f)
    }
}
impl fmt::Debug for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.__description())
    }
}

impl ::std::error::Error for StatusError {
    fn description(&self) -> &str {
        self.__description()
    }
}
