use std::fmt;


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

pub trait InitLanguage {
    fn init_language(&mut self);
}
