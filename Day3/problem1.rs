use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

static mut line_no : u32 = 0;
static mut total_col : u32 = 0;

fn main() {

     let mut ans = 0;
    // File input must exist in current path before this produces output
    
    if let Ok(lines) = read_lines("./input") {
    
    	let tree_vec: Vec<u32> = lines.map(is_tree).collect();
    	ans = tree_vec.iter().sum();
        
    }

    		println!("Answer: {}",ans);

    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_tree(s: std::result::Result<String,std::io::Error>) -> u32 {

	unsafe {
	if line_no == 0 {
		line_no += 1;
		total_col = s.unwrap().chars().count().try_into().unwrap();
		println!("Total column: {}",total_col);
		0 
	}
	else
	{ 
			let mut temp : u32 = 3*line_no;
			if temp > total_col {
				temp = temp % total_col;
				//Not below we do want the first character in this case
				//if temp == 0 {
					//temp = total_col-1;
				//}
			}
			
			let idx : usize = (temp).try_into().unwrap();
			
			//println!("Reading line {} and index {} !",line_no,idx );
			
			if s.unwrap().chars().nth(idx).unwrap() == '#'
			{
				//println!("Found a # at line {} and index {} !",line_no,idx );
				line_no += 1;
				1
			}
			else {
			line_no += 1;
			0
			}
	}
}
}