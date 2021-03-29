use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use jsonconv::jsonconv::rewrite_as_json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashSet;
use regex::Regex;
#[macro_use]
extern crate lazy_static;


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
	lazy_static! {
	static ref validvalecl :HashSet<&'static str> = {
	let mut eclvv = HashSet::new();
	
	eclvv.insert("amb");
	eclvv.insert("blu");
	eclvv.insert("brn");
	eclvv.insert("gry");
	eclvv.insert("grn");
	eclvv.insert("hzl");
	eclvv.insert("oth");
	eclvv
		};
	}
	// closure to check validity
	let check_validity = |passport : Passport| -> u32 {
		let mut retval: u32 = 0;
		if !validvalecl.contains(passport.ecl.as_str()) {
			return retval;
		}
		
		let byr: u32  = match passport.byr.trim().parse()
		{
			Ok(num) => num,
			Err(_)	=> 0,
		};
		
		if !((byr>=1920) && (byr<=2002))
		{
			return retval;
		}
		
		let iyr: u32  = match passport.iyr.trim().parse()
		{
			Ok(num) => num,
			Err(_)	=> 0,
		};
		
		if !((iyr>=2010) && (iyr<=2020))
		{
			return retval;
		}
		
		let eyr: u32  = match passport.eyr.trim().parse()
		{
			Ok(num) => num,
			Err(_)	=> 0,
		};
		
		if !((eyr>=2020) && (eyr<=2030))
		{
			return retval;
		}
		
		lazy_static! {
		
		static ref rehcl:Regex  = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
		
		}
		
		if !(rehcl.is_match(passport.hcl.as_str()))
		{
			return retval;
		}
		
		lazy_static! {
		static ref repid:Regex  = Regex::new(r"^[0-9]{9}$").unwrap();
		}
		
		if !(repid.is_match(passport.pid.as_str()))
		{
			return retval;
		}
		
		lazy_static! {
		static ref recm:Regex = Regex::new(r"^[0-9]{3}cm$").unwrap();
		}
		
		lazy_static! {
		static ref rein:Regex = Regex::new(r"^[0-9]{2}in$").unwrap();
		}
		
		let hgt = passport.hgt;
		if recm.is_match(hgt.as_str())
		{
			//check the number
			let mut orig_str = String::from(hgt);
			let cm_idx = orig_str.find("cm").unwrap();
			orig_str.split_off(cm_idx);
			let hgt: u32  = match orig_str.trim().parse()
			{
				Ok(num) => num,
				Err(_)	=> 0,
			};
			
			if(hgt<150||hgt>193)
			{
				return retval;
			}
			
		}
		else
		{
			if !rein.is_match(hgt.as_str())
			{
				return retval;
			}
			else
			{
				//check the number
				let mut orig_str = String::from(hgt);
				let cm_idx = orig_str.find("in").unwrap();
				orig_str.split_off(cm_idx);
				let hgt: u32  = match orig_str.trim().parse()
				{
					Ok(num) => num,
					Err(_)	=> 0,
				};
			
				if(hgt<59||hgt>76)
				{
					return retval;
				}
			}
		}
	
	 
		retval = 1;
		return retval;
	};

	let p: Result<Passport> =  serde_json::from_str(s.unwrap().as_str());
	
	let mut p = match p
	{	Ok(x) => ans = check_validity(x),
	  Err(e) => ans = 0,
	};
		
		
	ans
}