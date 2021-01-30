use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

static mut line_no : u32 = 0;
static mut total_col : u32 = 0;

#[derive(Debug)]
struct AllTrees(i32,i32,i32,i32,i32);

fn main() {

     let mut ans:i32 = 0;
    // File input must exist in current path before this produces output
    
    if let Ok(lines) = read_lines("./input") {
    
    	let tree_vec: Vec<AllTrees> = lines.map(is_tree).collect();
    	let mut line1 : i32 = 0;
    	let mut line3 : i32 = 0;
    	let mut line5 : i32 = 0;
    	let mut line7 : i32 = 0;
    	let mut linee : i32 = 0;
    	
    	for item in tree_vec {
    	//println!("The item is {:?}", item);
    	line1 += item.0;
    	line3 += item.1;
    	line5 += item.2;
    	line7 += item.3;
    	linee += item.4;
    	}
        
       println!("The values are {} , {} , {} ,{} , {} ", line1,line3,line5,line7,linee);
       //ans = line1*line3*line5*line7*linee; Needs i64
    }

    		
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_tree(s: std::result::Result<String,std::io::Error>) -> AllTrees {

	unsafe {
	if line_no == 0 {
		line_no += 1;
		total_col = s.unwrap().chars().count().try_into().unwrap();
		println!("Total column: {}",total_col);
		AllTrees(0,0,0,0,0) 
	}
	else
	{ 
			let mut temp1 : u32 = 1*line_no;
			let mut temp3 : u32 = 3*line_no;
			let mut temp5 : u32 = 5*line_no;
			let mut temp7 : u32 = 7*line_no;
			let mut tempeven : u32 = 0;
			
			let mut val1 : i32 = 0;
			let mut val3 : i32 = 0;
			let mut val5 : i32 = 0;
			let mut val7 : i32 = 0;
			let mut valeven : i32 = 0;												
			
			
			if temp1 >= total_col {
				temp1 = temp1 % total_col;
				
			}
			
			if temp3 > total_col {
				temp3 = temp3 % total_col;
				
			}
			
			if temp5 > total_col {
				temp5 = temp5 % total_col;
				
			}
			
			if temp7 > total_col {
				temp7 = temp7 % total_col;
				
			}
			
		
			
			
			let idx1 : usize = (temp1).try_into().unwrap();
			let idx3 : usize = (temp3).try_into().unwrap();
			let idx5 : usize = (temp5).try_into().unwrap();
			let idx7 : usize = (temp7).try_into().unwrap();
			
			//println!("Reading line {} and index {} !",line_no,idx1 );
			
			let s1 = s.unwrap().clone();
			let s3 = s1.clone();
			let s5 = s1.clone();
			let s7 = s1.clone();
			
			if (line_no % 2 == 0 )&&(line_no >1 ) {
			
				tempeven = line_no/2;
				if tempeven >= total_col {
				tempeven = tempeven % total_col;				
				}
				let idxeven : usize = (tempeven).try_into().unwrap();

				let seven = s1.clone();
				if seven.chars().nth(idxeven).unwrap() == '#'
				{
					valeven = 1;
				}
			else {
			
				valeven = 0;
				}
			}
			
			
				if s1.chars().nth(idx1).unwrap() == '#'
				{
					val1 = 1;
				}
				else {
			
				val1 = 0;
				}
				
				if s3.chars().nth(idx3).unwrap() == '#'
				{
					val3 = 1;
				}
				else {
			
				val3 = 0;
				}
				
				if s5.chars().nth(idx5).unwrap() == '#'
				{
					val5 = 1;
				}
				else {
			
				val5 = 0;
				}
				
				if s7.chars().nth(idx7).unwrap() == '#'
				{
					val7 = 1;
				}
				else {
			
				val7 = 0;
				}
				
			
			let returnstr = AllTrees(val1,val3,val5,val7,valeven);
			line_no += 1;
			returnstr
			
	}
}
}