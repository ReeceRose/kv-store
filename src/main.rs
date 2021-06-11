use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap(); //unwrap is not recommended.
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    //Database::new();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
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

        Ok(Database { map: map })
    }
}
