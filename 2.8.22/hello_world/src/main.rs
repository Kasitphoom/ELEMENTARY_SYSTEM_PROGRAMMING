use std::io::stdin;
fn main(){
   let mut line = String::new();
   println!("Enter your name :");
   let b1 = stdin().read_line(&mut line).unwrap();
   println!("Hello , {}", line);
   println!("no of bytes read , {}", b1);
}
