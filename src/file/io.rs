

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_site_file(path : &Path) -> Vec<(i32, i32)> {
    
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Couldn't open file!"), 
    };

    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Err(_) => panic!("Couldn't read file!"),
        _ => (),
    };

    let data : Vec<i32> = string.replace("(","")
        .replace(")","")
        .replace(","," ")
        .split_whitespace() //split the file by whitespace
        .filter_map( //execute lambdas on each split string part
            |s| match s.trim().parse() { //trim the string, parse the number
                Ok(t) => Some(t), //if parsing succeeds, return the number
                Err(e) => None,}) //if not, return nothing
        .collect(); //transform the filtermap to a vec

    let mut sites = Vec::new();
    for i in 0..data.len()/2 {
        sites.push((
            data[i*2],
            data[i*2 + 1],
        ));
    }

    sites
}

pub fn write_output_file(content : String) {
    let path = Path::new("output.txt");
    let mut file = match File::create(&path) {
        Err(why) => panic!("Oh no!"),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("Double oh no!"),
        Ok(_) => (),
    }
}