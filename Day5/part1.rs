use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

static mut max : u32 = 0;


fn main() {
    
     let (tx, rx) = mpsc::channel();
     let (tx1, rx1) = mpsc::channel();
     
    let handle1 =  thread::spawn(move || {
    
     if let Ok(lines) = read_lines("./input.txt") {
    
            // Consumes the iterator, returns an (Optional) String
        for line in lines {
       		tx.send(line.unwrap()).unwrap();
       		//thread::sleep(Duration::from_secs(2)); 
        }
        
      }
      });
      
      let handle2 = thread::spawn(move || {
       	
      	for received in rx {
       // println!("Got: {}", received);
        let seat: u32 = get_seat_id(received);
        tx1.send(seat).unwrap();
        
   			 }
      });
      
      let handle3 = thread::spawn(move || {
      	
      	for seat_id in rx1 {
        println!("Got: {}", seat_id);
        
	        unsafe {
	        if seat_id>max {
	        	max = seat_id;
	        	}
	        }
        
   			}
      });
      
      handle1.join().unwrap();
      handle2.join().unwrap();
      handle3.join().unwrap();
      
       unsafe {
      println!("Max is: {}",max);
      }
      
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_seat_id(line: String) -> u32 {
 let mut rows = line.clone();
 let colm = rows.split_off(7);
 //println!("Row is: {} and colm is {}",rows,colm);
 
 let row_val: u32 = get_range((0,127),rows);
 let colm_val: u32 = get_range((0,7),colm);
 
  row_val*8 + colm_val
}

//given the string
//iterate over it's characters
// if it's F or L - take lower half of the tuple
// if it's B or R - take uper half of the tuple
// when the string is single char take either the low or high
fn get_range((low,high) : (u32,u32), val : String) -> u32
{
	if val.len() == 1 {
		if(val=="F")||(val=="L") {
			 low
		}
		else
		{
			high
		}
	}
	else
	{
		let mut value = val.clone();
		let rem = value.split_off(1);
		let mid = (low+high+1)/2;
		if(value=="F")||(value=="L")
		{
			
			get_range((low,mid-1),rem)
		}
		else
		{
			get_range((mid,high),rem)
		}
		
	}
}