use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().expect("Value was not there");
    println!("The key is {} and value is {}", key, value);

    let mut database = Database::new().expect("Creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);

    match database.flush() {
        Ok(()) => println!("YAY!"),
        Err(err) => println!("OH NOS! Error! {}", err),
    }
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // Read the kv.db file
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };

        // Morally equivalent to adding "?"...
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        // Parse the string
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            // Populate the map
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map, flush: false })
    }

    // If rust screams at you:
    // 1. Immutable borrow first (&self)
    // 2. Mutable borrow (&mut self)
    // 3. self by ownership (self)
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    println!("We flushed");
    let mut contents = String::new();
    for (key, value) in &database.map {
        // let kvpair = format!("{}\t{}\n", key, value);
        // contents.push_str(&kvpair);

        // More verbose, but we aren't
        // creating a temporary variable <kvpair>
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(&&&&&&&&value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
    // todo!("Finish this method")
}
