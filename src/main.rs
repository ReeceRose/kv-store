use std::collections::HashMap;
use std::io::Error;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap(); //unwrap is not recommended.
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);

    let mut database = Database::new().expect("Failed to create database");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, Error> {
        let mut map = HashMap::new();
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Result::Err(error);
        //     }
        // };

        // read the file
        let contents = std::fs::read_to_string("kv.db")?; // The ? will pretty much produce the above commented out code

        for line in contents.lines() {
            // parse the string
            let (key, value) = line.split_once('\t').expect("Corrup database");
            // populate database
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // Flush will take ownership of database. This is by design.
    // Once flush is called, by design you can no longer use database
    // anymore and this is enfored by taking ownership here which
    // will clear database from memory after this method exits.
    // This is further enforced by the rust compiler.
    fn flush(self) -> Result<(), Error> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write("kv.db", contents)
    }
}
