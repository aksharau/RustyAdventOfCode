use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryFrom;

fn main() {

     // Have a mutating integer which is holding valid passwords
     let mut ans = 0;
    // File input must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                
                if is_valid(&ip) {
                println!("{}", ip);
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

    	    println!("Please input your guess.");

		    let mut guess = String::new();

            io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
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
let password = orig_str.split_off(colon_idx+1);//orig string is 1-3 a:
let len = orig_str.len();
orig_str.truncate(len-1);//need to remove the colon//orig string is 1-3 a
//println!("The string after : {}",orig_str);

let space_idx = orig_str.find(' ').unwrap();
let  criteria = orig_str.split_off(space_idx+1);//orig string is 1-3 , criteria is a

//println!("The string space : {}",orig_str);

let hyphen_idx= orig_str.find('-').unwrap();
let  max = orig_str.split_off(hyphen_idx+1);//max is -3
let len2 = orig_str.len();
 orig_str.truncate(len2-1);
let mut x1: u32 = orig_str.trim().parse().expect("Please type a number!");
let mut y1: u32 = max.trim().parse().expect("Please type a number!");
//let c = password.matches(&criteria).count();

let chars: Vec<char> = password.chars().collect();
let crit: Vec<char> = criteria.chars().collect();

x1 = x1;
y1 = y1;
println!("x {}, y {}, crit{} ", x1, y1,crit[0] );
let x = usize::try_from(x1).unwrap();
let y = usize::try_from(y1).unwrap();


if ((chars[x]==crit[0])&&(chars[y]!=crit[0]))||((chars[x]!=crit[0])&&(chars[y]==crit[0])) {
	println!("x {}, y {}, crit{} , {}, {}", x, y,crit[0],chars[x],chars[y] );
}
((chars[x]==crit[0])&&(chars[y]!=crit[0]))||((chars[x]!=crit[0])&&(chars[y]==crit[0]))

}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
