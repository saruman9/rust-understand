extern crate understand;

use std::env;
use std::io::Write;

use understand::db::Db;

fn main() {
    if  env::args().count() != 2 {
        writeln!(&mut std::io::stderr(), "Expected one argument - path to UDB database.")
            .expect("Failed write to stderr.");
        std::process::exit(2);
    }

    let udb_path: &str = &env::args().nth(1).unwrap();

    // Open UDB database.
    let udb_db: Db = Db::open(udb_path).expect("Error of open database.");
    println!("Name: {}\nVersion: {}", udb_db.name(), udb_db.version());

    // Get entities.
    let ents = udb_db.entities();
    if let Some(mut ents) = ents {
        ents.sort_by_key(|ent| ent.name_long());
        for ent in &ents {
            println!("{}", ent);
            // Get references of entity.
            if let Some(mut refs) = ent.references() {
                println!("REFERENCES:");
                refs.sort_by_key(|refer| refer.entity().name_long());
                for refer in &refs {
                    println!("{}", refer);
                }
            }
            println!("--------------------------------------------------------------------------------");
        }
    } else {
        writeln!(&mut std::io::stderr(), "Database is empty!")
            .expect("Failed write to stderr.");
    }
}
