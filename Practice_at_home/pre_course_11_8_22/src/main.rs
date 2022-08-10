#![allow(dead_code)]

fn use_array(){

    // Arrays must have a known length and all elements must be initialized
    let _a:[i32;5] = [1, 2, 3, 4, 5];
    let _a2:[i32;3] = [0; 3]; // [0, 0, 0]

    // Unlike arrays the length of a slice is determined at runtime
    let _slice = &_a[1 .. 3];

    println!("{:?}", _a);

}

fn use_math() {

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);

}

#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn use_struct(){

    // adding values
    let name = String::from("Phoom");
    let age = 17;
    let phoom = Person{name, age};

    println!("{:?}", phoom);

    let point = Point{x: 10.0, y: 11.2};

    println!{"{:?}", point};
    
}

fn main() {
    
    use_struct();

}
