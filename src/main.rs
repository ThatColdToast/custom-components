use std::{collections::HashSet, path::PathBuf};

fn main() {
    let paths = std::fs::read_dir("./pages/").unwrap();

    for path in paths {
        let mut in_flight: HashSet<String> = HashSet::new();
        let filepath = path.unwrap().path();

        println!("Page: {}", &filepath.display());

        resolve(&mut in_flight, &filepath.to_str().unwrap().to_string());

        //if in_flight.contains(&filepath.to_str().unwrap()) {
        //    return;
        //}

        //in_flight.insert(&filepath.to_str().unwrap());
        //let filedata = std::fs::read_to_string(&filepath.to_str().unwrap()).unwrap();
        //println!("{}", filedata);
    }
}

fn resolve(in_flight: &mut HashSet<String>, filepath: &String) -> String {
    if in_flight.contains(filepath) {
        panic!("custom-components does not support recursive types");
    }

    in_flight.insert(filepath.to_string());
    let filedata = std::fs::read_to_string(&filepath).unwrap();
//    println!("{}", &filedata);  
//    filedata.find("use");
    String::from(&filedata)
}
