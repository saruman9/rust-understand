extern crate understand_sys;

use understand_sys::{UdbReference, UdbKind};

use language::Language;

pub struct Entity<'ent> {
    pub id         : i32,
    pub name       : &'ent str,
    pub shortname  : &'ent str,
    pub longname   : &'ent str,
    pub contents   : &'ent str,
    pub references : Vec<UdbReference>,
    pub kind       : UdbKind,
    pub language   : Language,
}
