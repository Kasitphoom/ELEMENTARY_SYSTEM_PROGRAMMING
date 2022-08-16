
use std::fs::File;
use std::io::Write;
use chrono::prelude::*;

fn write_my_name(my_name:String, s_id:i64) -> std::io::Result<()> {
	let mut f = File::create("name1.txt")?;
	let local: DateTime<Local> = Local::now();
	let utc: DateTime<Utc> = Utc::now();
	let year = utc.year();
	let date = utc.day();
	let month = utc.month();
	// Make a string to add to your file
	// build the string with format
	let contents = format!("{} {}\n{}/{}/{}", my_name, s_id, date, month, year);
	// Debug .. check the content
        println!("Contents <<{}>>", contents );
	// Write the content to a file
	let _e = if let Err(_e) = 
                write!(f,"{}", contents) { _e } 
		else { 
            println!(".. else <ignore the panic!!>"); 
		    todo!() 
		};
	Ok(())
}

fn main() {
        println!("\nVersion 1");
	let my_name = String::from("\u{0E01}\u{0E29}\u{0E34}\u{0E15}\u{0E20}\u{0E39}\u{0E21}\u{0E34}");
	println!("My name is {}", my_name);
	let s_id:i64 = 65011328;
      	let content = write_my_name(my_name, s_id).unwrap();
	println!("Returned result {:?}", content);
}

