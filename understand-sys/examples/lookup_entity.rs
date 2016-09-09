extern crate understand_sys;

use std::ffi::{CString, CStr};
use std::env;
use std::mem;

use understand_sys::{UdbEntity, udbDbOpen, udbStatusText, udbLookupEntity, udbEntityNameLong,
                     udbKindLongname, udbEntityKind, udbDbClose, udbListEntityFree};

fn main() {
    if env::args().len() != 5 {
        panic!("Expected four arguments - {} path_UDB filter_name filter_kind \
                search_in_shortname.",
               env::args().nth(0).unwrap());
    }
    let udb_path_str = CString::new(env::args().nth(1).unwrap()).unwrap().as_ptr();
    let name_str = CString::new(env::args().nth(2).unwrap()).unwrap().as_ptr();
    let kind_str = CString::new(env::args().nth(3).unwrap()).unwrap().as_ptr();
    let search_in_shortname = env::args().nth(4).unwrap().parse::<i32>().unwrap();

    unsafe {
        let udb_status = udbDbOpen(udb_path_str);
        let udb_status_str = CStr::from_ptr(udbStatusText(udb_status)).to_string_lossy();
        if udb_status as u8 != 0 {
            panic!("Unexpected status of open UDB database: {}(err {})",
                   udb_status_str,
                   udb_status as u8);
        }

        let mut udb_list_ents: *mut UdbEntity = mem::uninitialized();
        let mut udb_count_ents = 0;

        udbLookupEntity(name_str,
                        kind_str,
                        search_in_shortname,
                        &mut udb_list_ents,
                        &mut udb_count_ents);

        for i in 0..udb_count_ents {
            let ent_raw = *udb_list_ents.offset(i as isize);
            let ent_name_long = CStr::from_ptr(udbEntityNameLong(ent_raw)).to_string_lossy();
            let ent_kind_raw = udbEntityKind(ent_raw);
            let ent_kind_name_long = CStr::from_ptr(udbKindLongname(ent_kind_raw))
                .to_string_lossy();
            println!("{} - {}", ent_name_long, ent_kind_name_long);
        }

        udbListEntityFree(udb_list_ents);
        udbDbClose();
    }
}
