use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;



fn main() {
    // File input must exist in current path before this produces output
    let mut set: HashSet<i32> = HashSet::new();

    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                let x = ip.parse().expect("Not a number!");
                set.insert(x);
            }
            else
            {
            	println!("Not able to read")
            }
        }
    }

    //Part 1 is just the inner for loop
    for first in set.iter(){
   	 for x in set.iter() {
   	 		if  x == first {
   	 			continue;
   	 		}
   		 	let rem = 2020 - (first+x);
    		if set.contains(&rem) {
    		let ans = rem*x*first;
    		println!("Answer: {}",ans);
    		break;
    		}

		}
	}

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
