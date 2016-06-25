extern crate understand_sys;

use std::fmt;

pub mod status;
pub mod language;
pub mod db;
pub mod entity;
pub mod kind;
pub mod library;
pub mod reference;

#[derive(Debug)]
pub enum Error {
    IntoString(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IntoString(ref msg) => write!(f, "Convert to String error: {}", msg),
        }
    }
}

