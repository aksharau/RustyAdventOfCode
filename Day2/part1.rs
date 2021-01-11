use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

     // Have a mutating integer which is holding valid passwords
     let mut ans = 0;
    // File input must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                
                if is_valid(&ip) {
                ans += 1;
                }
            }
            else
            {
            	println!("Not able to read")
            }
        }
    }

    		println!("Answer: {}",ans);

    	 
}

//Spilt the string based on first occurence of :
//1-3 a: abcde
//Store the second part of the split in password string
// Then split the first part based on space
//store the criteria char in a variable
//Split the first part on - and store the resultant into max and min values
//Count the number of occurences of criteria in password
//if the values lies in the range return true else return false
fn is_valid(s: &String) -> bool {
let mut orig_str = String::from(s);
let colon_idx = s.find(':').unwrap();
let password = orig_str.split_off(colon_idx+1);
let len = orig_str.len();
orig_str.truncate(len-1);//need to remove the colon


let space_idx = orig_str.find(' ').unwrap();
let  criteria = orig_str.split_off(space_idx+1);



let hyphen_idx= orig_str.find('-').unwrap();
let  max = orig_str.split_off(hyphen_idx+1);//max is -3
let len2 = orig_str.len();
 orig_str.truncate(len2-1);
let x: usize = orig_str.trim().parse().expect("Please type a number!");
let y: usize = max.trim().parse().expect("Please type a number!");
let c = password.matches(&criteria).count();
(x <= c) &&( c<= y)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
