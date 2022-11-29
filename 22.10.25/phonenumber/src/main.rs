use std::io;

#[derive(Debug)]
enum Type {Local, International, LikelyScammer, Special, Invalid}
enum CardType {Mastercard, Visa, Others}

#[derive(Debug)]
struct TelNo { 
    tel_num: u64,
    tel_type: Type,
    code: String,
}

struct Card{
  card_no: u64,
  card_type: CardType,
}

fn print_telNo( tn: TelNo ) { 
  println!("({}) {} => {:?}", tn.code, tn.tel_num, tn.tel_type );
}

fn read_text_line() -> String {
  eprint!("Enter a telephone number ");
  let mut buffer = String::new();
  let _result = io::stdin().read_line(&mut buffer);
  buffer = buffer.trim().to_string();
  eprintln!("Buffer read ({}) [{}]", buffer.len(), buffer );
  buffer
}

// Some other useful functions, eg
fn print_num(tel: TelNo){
    println!("({}) {}",tel.code, tel.tel_num);
}
fn check_valid( vec: Vec<String>, digit: usize ) -> bool {
    let mut valid = false;
    let mut tel_num = String::new();
    for i in 0..vec.len(){
        tel_num.push_str(&vec[i]);
    }
    // remove the first char
    tel_num.remove(0);

    if tel_num.len() == digit{
        valid = true;
    }
    valid
}

fn proc_text_line( s: String ) -> TelNo {
  let next_string: String = String::new();
  let mut nc:u64 = 0;
  let mut tel_vec = Vec::new();
  let mut tel_text = String::new();
  let mut tel_num = String::new();
  let mut tel_type = Type::Invalid;
  let mut tn = TelNo{ tel_num : 0, tel_type: Type::Local, code: "".to_string() };
  // Ignore all irrelevant digits!!!
  for ch in s.chars() {
    // Do somehting useful here
    if ch != ' '{
        tel_text.push(ch);
    }
    else{
        tel_vec.push(tel_text);
        tel_text = String::new();
    }
    
    nc += 1;
  }
  tel_vec.push(tel_text);
  eprintln!("{:?}", tel_vec);
  if tel_vec[0].chars().nth(0).unwrap() == '+'{
    if !check_valid(tel_vec.clone(), 11){
        return tn
    }
  }
  else if tel_vec[0].chars().nth(0).unwrap() == '0' {
    if !check_valid(tel_vec.clone(), 9){
        return tn
    }
  }
  else if tel_vec[0].chars().nth(0).unwrap() != '0'{
    if !check_valid(tel_vec.clone(), 9){
        return tn
    }
  }

  let mut code = String::new();
  if tel_vec[0] == "+66" || tel_vec[0].chars().nth(0).unwrap() == '0'{
    tel_type = Type::Local;
    code = "+66".to_string();
  }
  else if tel_vec[0] == "+657"{
    tel_type = Type::LikelyScammer;
    code = "+657".to_string();
  }
  else if tel_vec[0].chars().nth(0).unwrap() == '+'{
    tel_type = Type::International;
    code = tel_vec[0].clone();
  }
  else{
    tel_type = Type::Special;
  }

  tel_num = "0".to_string();
  for i in 1..tel_vec.clone().len(){
    tel_num.push_str(&tel_vec[i]);
  }
  tn = TelNo{ tel_num :  tel_num.parse::<u64>().unwrap(), tel_type: tel_type, code: code };
  eprintln!("Char processed {}", nc );
  tn
}

fn main() {
  println!("Telephone numbers");
  let mut cnt: u64 = 0;
  
  // Check each phone number
  loop {
    let mut text = read_text_line();
    eprintln!("Text [{}]", text );
    if text.len() < 3 { break; }
    else {
      let nc = proc_text_line( text );
      eprintln!("{:?}", nc );
      print_telNo( nc );
      cnt += 1;
    }
  }
  eprintln!("\n{} tel nos processed", cnt );
  // Check credit card numbers
  // ...
}
