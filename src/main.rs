use std::{collections::HashSet, borrow::Borrow};

fn main() {
    let paths = std::fs::read_dir("./pages/").unwrap();

    let mut in_flight: HashSet<&str> = HashSet::new();

    for path in paths {
        println!("Page: {}", path.as_ref().unwrap().path().display());
        if in_flight.contains(path.as_ref().unwrap().path().to_str().unwrap()) {
            return;
        }
        
        in_flight.insert(path.as_ref().unwrap().path().to_str().unwrap());
        let file = std::fs::read_to_string(path.unwrap().path()).unwrap();
        
    }
}
