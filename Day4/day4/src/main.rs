use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use jsonconv::jsonconv::rewrite_as_json;
use serde::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Serialize, Deserialize)]
struct Passport {
    ecl: String,
    pid: String,
    eyr: String,
    hcl: String,
    byr: String,
    iyr: String,
    cid: Option<String>,
    hgt: String,
   
}

fn main() {
     let mut ans = 0;
    
     rewrite_as_json();
     if let Ok(lines) = read_lines("./out.txt") {
     	let pass_vec: Vec<u32> = lines.map(is_valid_passport).collect();
     	ans = pass_vec.iter().sum();
     }
    
    println!("Answer: {}",ans);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_valid_passport(s: std::result::Result<String,std::io::Error>) -> u32 {

	let mut ans = 0;

	let p: Result<Passport> =  serde_json::from_str(s.unwrap().as_str());
	
	let mut p = match p
	{	Ok(_) => ans = 1,
	  Err(e) => ans = 0,
	};
	
		
	ans
}