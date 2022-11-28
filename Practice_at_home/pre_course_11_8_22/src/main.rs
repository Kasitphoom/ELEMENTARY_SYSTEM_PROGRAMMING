use std::any::Any;
use std::process::*;

use rand::Error;

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
struct Point{
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone)]
struct Triangle{
    a: Point,
    b: Point,
    c: Point,
}

#[derive(Debug, Copy, Clone)]
struct Rectangle{
    a: Point,
    b: Point,
}

trait Shape{
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

impl Shape for Triangle{
    fn area(&self) -> f64{
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let ab = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
        let bc = ((c.x - b.x).powi(2) + (c.y - b.y).powi(2)).sqrt();
        let ca = ((a.x - c.x).powi(2) + (a.y - c.y).powi(2)).sqrt();
        let s = (ab + bc + ca) / 2.0;
        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let ab = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
        let bc = ((c.x - b.x).powi(2) + (c.y - b.y).powi(2)).sqrt();
        let ca = ((a.x - c.x).powi(2) + (a.y - c.y).powi(2)).sqrt();
        ab + bc + ca
    }
}

impl Shape for Rectangle{
    fn area(&self) -> f64 {
        let a = self.a;
        let b = self.b;
        let ab = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
        let bc = ((b.x - a.x).powi(2) + (a.y - b.y).powi(2)).sqrt();
        ab * bc
    }

    fn perimeter(&self) -> f64 {
        let a = self.a;
        let b = self.b;
        let ab = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
        let bc = ((b.x - a.x).powi(2) + (a.y - b.y).powi(2)).sqrt();
        2.0 * (ab + bc)
    }
}

fn divide_option(a: f64, b: f64) -> Option<f64>{
    if b == 0.0{
        None
    }else{
        Some(a / b)
    }
}

fn divide_result(a: f64, b: f64) -> Result<f64, String>{
    if b == 0.0{
        Err("Division by zero".to_string())
    }else{
        Ok(a / b)
    }
}

fn main(){

    let p1 = Point{x: 1.0, y: 2.0};
    let p2 = Point{x: 2.0, y: 1.0};
    let p3 = Point{x: 3.0, y: 2.0};

    println!("p1 < p2: {}", p1 < p2); // true
    println!("p1 > p2: {}", p1 > p2); // false
    println!("p1 == p2: {}", p1 == p2); // false
    println!("p1 != p2: {}", p1 != p2); // true

    let triangle = Triangle{a: p1, b: p2, c: p3};
    println!("Triangle Area: {}", triangle.area());

    let rectangle = Rectangle{a: p1, b: p2};
    println!("Rectangle Area: {}", rectangle.area());

    let devide_option1 = divide_option(1.0, 2.0); // return Some(0.5)
    let devide_option2 = divide_option(1.0, 0.0); // return None

    let devide_result1 = divide_result(1.0, 2.0); // return Ok(0.5)
    let devide_result2 = divide_result(1.0, 0.0); // return Compile Error

    println!("{:-^15}|{:-^15}", "Number", "Count");
    for i in 1..=10{
        println!("{:^15}|{:<15b}", i, i);
    }

    let str = "Hello World";
    let str1 = "Hello world 2";
    const ALLOCATE:usize = 10;

    println!("str:{str:>8.5}");

    println!("{str:0$} -> {str1:0$}", ALLOCATE);

    match devide_option2{
        Some(x) => println!("devide_option1: {}", x),
        None => println!("devide_option1: None"),
    }

    let mut vec = vec![1, 2, 3, 4, 5];
    // push at index
    vec.insert(2, 6);
    println!("{:?}", vec);
}