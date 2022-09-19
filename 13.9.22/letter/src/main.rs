use std::{io, vec, env, fs};
use std::io::{BufWriter, Write};
use std::time::{Instant, Duration};

#[derive(Debug, Clone, Copy)]
struct freq_table_entry {
    letter: char,
    count: u64,
    frequency: f64,
}

// This function will update your freq_table
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

fn calc_percentage( freq_table: &mut Vec<freq_table_entry>, total: u64 ) {
  for e in freq_table.iter_mut() {
    e.frequency = ((e.count as f64) / (total as f64)) * 100.0;
  }
}

fn print_vec( out_file: &String, freq_table: &Vec<freq_table_entry> , non_thai_table: &Vec<freq_table_entry>, text: String, total: u64, duration: Duration ) {
  // create a new file
  let file = fs::File::create(out_file).expect("Unable to create file");
  // write to the file
  let mut file = BufWriter::new(file);

  writeln!(file, "Text to be processed:\n{}\n\n", text).expect("Unable to write data");
  writeln!(file, "========================================================\n/***************Thai Character***************\\\n").expect("Unable to write to file");
  writeln!(file, "Letter\tUnicode\tCount\tFrequency (%)").expect("Unable to write to file");

  for e in freq_table.iter() {
    let unicode = format!("0{:X}", e.letter as u64);
    writeln!(file, "{:9}\t0{:8}\t{}\t{:.3}%\n", e.letter, unicode, e.count, e.frequency ).expect("Unable to write to file");
  }
  writeln!(file, "========================================================\n/***************Non-Thai Character***************\\\n").expect("Unable to write to file");
  writeln!(file, "Letter\tUnicode\tCount\tFrequency (%)").expect("Unable to write to file");

  for e in non_thai_table.iter() {
    let unicode = format!("0{:X}", e.letter as u64);
    writeln!(file, "{:9}\t0{:8}\t{}\t{:.3}%\n", e.letter, unicode, e.count, e.frequency ).expect("Unable to write to file");
  }
  
  let (max_thai, max_non_thai) = add_summary(freq_table, non_thai_table);
  let time = format!("Time duration to {} miliseconds", duration.as_millis());
  let total = format!("Total number of characters {}", total);

  let sum_head = format!("RESULT SUMMARY: ");
  writeln!(file, "========================================================\n{}\n{}\n{}\n{}\n{}",sum_head, max_thai, max_non_thai, time, total).expect("Unable to write to file");

} 

fn add_summary(freq_table: &Vec<freq_table_entry>, non_thai_table: &Vec<freq_table_entry>) -> (String, String) {

  // check most frequent letter in thai
  let mut max = 0;
  let mut max_letter = ' ';
  for e in freq_table.iter() {
    if e.count > max {
      max = e.count;
      max_letter = e.letter;
    }
  }
  let max_thai = format!("Most frequent letter in Thai is \"{}\" with {} occurrences", max_letter, max);

  // check most frequent letter that are not thai
  max = 0;
  max_letter = ' ';
  for e in non_thai_table.iter() {
    if e.count > max {
      max = e.count;
      max_letter = e.letter;
    }
  }
  let max_non_thai = format!("Most frequent letter that are not Thai is \"{}\" with {} occurrences", max_letter, max);

  return(max_thai, max_non_thai);
}


fn main() {
  // get time duration of the program
  let start = Instant::now();
  let args: Vec<String> = env::args().collect();
  let filepath = &args[1];
  let outputpath = &args[2];

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
    
 
    nc = unpack_text_line( l.to_string(), &mut cha, &mut non_thai );
    ix += 1;
    
    total += nc;
  }
  // stop the timer
  let duration = start.elapsed();
  // fn to Generate frequency fractions
  calc_percentage(&mut cha, total);
  calc_percentage(&mut non_thai, total);
  // fn to Generate report
  print_vec( outputpath ,&cha, &non_thai, text, total, duration);
}