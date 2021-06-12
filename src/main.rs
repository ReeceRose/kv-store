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
    database.flush().unwrap(); // Call to flush is not required. Will be picked up by drop()
}

struct Database {
    map: HashMap<String, String>,
    flushed: bool,
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

        Ok(Database {
            map,
            flushed: false,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> Result<(), Error> {
        self.flushed = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flushed {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> Result<(), Error> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
