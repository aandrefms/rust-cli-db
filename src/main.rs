use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    println! ("Key {} and Value {}", key, value);
    let contents = format!("{}\t{}\n", key, value);
    let write_result = std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Database::new() crashed");
}

struct Database {
    map: HashMap<String, String>,
    testing: String,
    testing2: String,

}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file
/*         let contents = match std::fs::read_to_string( "kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Result::Err(error); // Or Just Err(error)
            }
        }; */ // This can be replaced with the sintaxe below (because it's such a common thing in rust) using '?'
        let mut mymap = HashMap::new();
        let contents: String = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            //let pair = line.split_once('\t').expect("Corrupt Database");
            let (key, value) = line.split_once('\t').expect("Corrupt Database");
            mymap.insert(key.to_owned(),value.to_owned());
        }
        // In order to pass ownership(string slice -> string) -> to_owned(), String::from, 
        // parse the string
        Ok(Database {
            map: mymap,
            testing: "ola".to_owned(),
            testing2:"ola".to_string()
        })
    }
}
