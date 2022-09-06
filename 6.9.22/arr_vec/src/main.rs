// use chrono::prelude::*;
// use chrono::{DateTime,Local};
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone)]
struct Point {
  x: f64,
  y: f64,
}

#[derive(Debug, Copy, Clone)]
struct Kite {
  p1: Point,
  p2: Point,
  p3: Point,
  p4: Point,
} 

fn time_diff_nsecs( t0:Instant ) ->  f64 {
	let duration = t0.elapsed();
	let d_nsecs = duration.as_nanos();
	d_nsecs as f64
}

const N:usize = 1000;

fn time_array(n:usize) -> f64 {
  // Initialize array
  let p0 = Point{x:0.0,y:3.0};
  let p1 = Point{x:2.0,y:0.0};
  let p2 = Point{x:-2.0,y:0.0};
  let p3 = Point{x:0.0,y:7.0};

  let mut c0 = Kite{p1: p0, p2: p1, p3: p2, p4: p3};
  let mut v_array: [Kite; N] = [c0; N];
  let t0 = Instant::now();
  for j in 0..n {
    v_array[j] = c0;
    c0.p4.y *= 1.001;
  }
  let duration = time_diff_nsecs(t0);
  println!("Start {:?} Time to initialize array {:?}", t0, duration);
  array_access_noi(v_array);
  array_access_i(v_array);
  duration as f64
}

fn array_access_i(arr:[Kite; N]) -> f64{ 
  let t0 = Instant::now();
  for i in arr{
    let mut a = i;
  }
  let duration = time_diff_nsecs(t0);
  println!("Start {:?} Time to access array with iterators {:?} ns/elm", t0, (duration)/(N as f64));
  duration as f64
}

fn array_access_noi(arr:[Kite; N]) -> f64{ 
  let t0 = Instant::now();
  for i in 0..N{
    let mut a = arr[i];
  }
  let duration = time_diff_nsecs(t0);
  println!("Start {:?} Time to access array without iterators {:?}  ns/elm", t0, (duration)/(N as f64));
  duration as f64
}

fn vector_access_i(arr:Vec<Kite>) -> f64{ 
  let t0 = Instant::now();
  for i in arr{
    let mut a = i;
  }
  let duration = time_diff_nsecs(t0);
  println!("Start {:?} Time to access vector with iterators {:?} ns/elm", t0, (duration)/(N as f64));
  duration as f64
}

fn vector_access_noi(arr:Vec<Kite>) -> f64{ 
  let t0 = Instant::now();
  for i in 0..N{
    let mut a = arr[i];
  }
  let duration = time_diff_nsecs(t0);
  println!("Start {:?} Time to access vector without iterators {:?}  ns/elm", t0, (duration)/(N as f64));
  duration as f64
}

fn time_vector(n:usize) -> f64 {
  // Initialize vector
  let p0 = Point{x:0.0,y:3.0};
  let p1 = Point{x:2.0,y:0.0};
  let p2 = Point{x:-2.0,y:0.0};
  let p3 = Point{x:0.0,y:7.0};
  
  let mut c0 = Kite{p1: p0, p2: p1, p3: p2, p4: p3};
  let mut v_objects:Vec<Kite> = Vec::with_capacity(150);
  let t0 = Instant::now();
  let mut cc = v_objects.capacity();
  let mut last_cc = v_objects.capacity();
  // println!("Start {:?}", t0 );
  for j in 0..n {
    v_objects.push(c0);
    c0.p4.y *= 1.001;
    cc = v_objects.capacity();
    if cc > last_cc{
      println!("{:?}", cc);
      last_cc = cc;
    }
  }
  let duration = time_diff_nsecs(t0);
  println!("Start {:?} Time to initialize vector {:?}", t0, duration);
  vector_access_i(v_objects.clone());
  vector_access_noi(v_objects.clone());
  duration as f64
}

fn main() {
  println!("Vectors - Performance and Timing");

  print!("\n=============================================================================\nArray {} elements total\n", N);
  let t_per_array = time_array(N);
  println!("Array {} elements total {} ns mean {} ns/elem", N, t_per_array, (t_per_array)/(N as f64));
  print!("\n=============================================================================\nVector {} elements total\n", N);
  let t_per_vector = time_vector(N);
  println!("Vector {} elements total {} ns mean {} ns/elem", N, t_per_vector, t_per_vector/(N as f64));
 
  //** Edit your code or ADD more functions to get the other times **/
  //* Make another function that uses an iterator **/

}
