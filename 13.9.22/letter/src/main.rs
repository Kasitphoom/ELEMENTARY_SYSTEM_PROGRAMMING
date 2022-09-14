use std::{io, vec, env, fs};
use std::io::{BufWriter, Write};

#[derive(Debug, Clone, Copy)]
struct freq_table_entry {
    letter: char,
    count: u64,
    frequency: f64,
}

// This function will update your freq_table ...
// maybe it needs an extra argument
fn unpack_text_line( line: String, freq_table: &mut Vec<freq_table_entry>, non_thai_table: &mut Vec<freq_table_entry> ) -> u64 {
  let mut nc: u64 = 0;
  for i in line.chars() {
    if (i as u32) > 0x0E00 && (i as u32) < 0x0E7F{
      check_add_letter( i, freq_table );
      nc += 1;
    }
    else{
      check_add_letter( i, non_thai_table );
      nc += 1;
    }
  }
  nc
}

fn read_text_line(filepath: &String) -> String{
  print!("Text to be processed ");
  let mut buffer = fs::read_to_string(filepath).expect("Unable to read file");
  buffer = buffer.trim().to_string();
  eprintln!("Buffer ({}) [{}]", buffer.len(), buffer );
  buffer
}

fn check_add_letter( letter: char, freq_table: &mut Vec<freq_table_entry> ) {
  let mut found: bool = false;
  for e in freq_table.iter_mut() {
    if e.letter == letter {
      e.count += 1;
      found = true;
      break;
    }
  }
  if !found {
    let new_entry = freq_table_entry {
      letter: letter,
      count: 1,
      frequency: 0.0,
    };
    freq_table.push( new_entry );
  }
}

// Some other useful functions, eg
// calc_fractions -- to calculate the fracctions for each letter
fn calc_percentage( freq_table: &mut Vec<freq_table_entry>, total: u64 ) {
  for e in freq_table.iter_mut() {
    e.frequency = ((e.count as f64) / (total as f64)) * 100.0;
  }
}

fn print_vec( out_file: &String, freq_table: &Vec<freq_table_entry> , non_thai_table: &Vec<freq_table_entry>) {
  // create a new file
  let file = fs::File::create(out_file).expect("Unable to create file");
  // write to the file
  let mut file = BufWriter::new(file);

  println!("Thai Character\n========================================================");
  write!(file, "Thai Character\n========================================================\n").expect("Unable to write to file");

  println!("Letter\tUnicode\tCount\tFrequency (%)");
  write!(file, "Letter\tUnicode\tCount\tFrequency (%)\n").expect("Unable to write to file");

  for e in freq_table.iter() {
    println!("{}\t0{:X}\t{}\t{:.3}%", e.letter, e.letter as u64, e.count, e.frequency );
    write!(file, "{:5}\t0{:X}\t{}\t{:.3}%\n", e.letter, e.letter as u64, e.count, e.frequency ).expect("Unable to write to file");
  }
  println!("\nNon-Thai Character\n========================================================");
  write!(file, "\nNon-Thai Character\n========================================================\n").expect("Unable to write to file");

  println!("Letter\tUnicode\tCount\tFrequency (%)");
  write!(file, "Letter\tUnicode\tCount\tFrequency (%)\n").expect("Unable to write to file");

  for e in non_thai_table.iter() {
    println!("{}\t0{:X}\t{}\t{:.3}%", e.letter, e.letter as u64, e.count, e.frequency );
    write!(file, "{:5}\t0{:X}\t{}\t{:.3}%\n", e.letter, e.letter as u64, e.count, e.frequency ).expect("Unable to write to file");
  }


} 
// print_report - print table in readable form
// add_summary - add useful summary - # useful chars, #other spaces, punctuation, non-Thai letter
//     Most frequent letter

fn main() {
  let args: Vec<String> = env::args().collect();
  let filepath = &args[1];
  let outputpath = &args[2];

  println!("Character Frequencies");
  let mut ix: u64 = 0;
  let mut total = 0;
  let mut nc = 0;
  // Construct and empty table
  let mut cha:Vec<freq_table_entry> = Vec::new();
  let mut non_thai:Vec<freq_table_entry> = Vec::new();
  // loop through the text
  let text = read_text_line(filepath);
  let lines = text.lines();
  for l in lines {
    
    eprintln!("Text [{}], len {}", l , l.len());
 
    nc = unpack_text_line( l.to_string(), &mut cha, &mut non_thai );
    ix += 1;
    
    total += nc;
  }
  eprintln!("\n{} lines processed", ix );
  eprintln!("{} characters processed", total );
  // fn to Generate frequency fractions
  calc_percentage(&mut cha, total);
  calc_percentage(&mut non_thai, total);
  // fn to Generate report
  print_vec( outputpath ,&cha, &non_thai );
  // fn to add summary
  // ...
}