
pub mod jsonconv {
	
	use std::fs::File;
	use std::io::prelude::*;
	use std::io::{self, BufRead};
	use std::path::Path;

	
		// Read lines, concatinate to previous line
		// Untill an empty line is read
		// On reading empty line or the last lineof the file - if the last line is non-empty concatinate to previous
		// Write the previous one to another file with json format per line
		// We will split with space and then with : and re write as JSON string


	pub fn rewrite_as_json() {
	
		let mut outfile = File::create("out.txt").unwrap();
		let mut passport = String::from("");
	
		if let Ok(lines) = read_lines("./input") {
		for line in lines {
            if let Ok(ip) = line {
            		if ip == "" {
            			let str = get_as_json(&passport);
            			outfile.write_all(str.as_bytes()).expect("Unable to write data");
            			passport.clear();
            		 }
            		else
            		{
            			passport.push_str(&ip);
            		}
                println!("Current line {} passport {}", ip,passport);
             }
                
					}
			}	
	
	
	 }
	
	fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
	}
	
	// We will split with space and then with : and re write as JSON string
	//Example : ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
	
	pub fn get_as_json(s: &String) -> String {
	let  orig_str  = String::from(s);
	let mut return_str = String::from("{");
	
	let  sentence = orig_str.split_whitespace(); 
	
	
	let mut isfirstword = true;
	for word in sentence {
		
		
		let keyval = word.split(":");
		let mut key = true;
		for item in keyval {
			
			if key {
				if !isfirstword {
					return_str.push_str(" , ");
				}
			}
	
			return_str.push_str("\"");
			return_str.push_str(item);
			return_str.push_str("\"");
			if key {
				return_str.push_str(":");
				key = false;
			}
			
			
			if isfirstword {
				isfirstword = false;
			}
		}
	}
	return_str.push_str("}");
	return_str
	}

}


#[cfg(test)]
mod tests {
		use super::*;
		
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn jsonify_works() {
    let str = String::from("ecl:gry pid:860033327 eyr:2020");
    let expected = r#"{"ecl":"gry" , "pid":"860033327" , "eyr":"2020"}"#;
    let ans = jsonconv::get_as_json(&str);
    assert_eq!(ans,expected);
    }
}
