#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use libm::*;

#[derive(Debug, Copy, Clone)]
struct Point {
   x: f64,
   y: f64,
}

impl Point {
    fn new() -> Point {
        Point{x: 0.0, y: 0.0}
    }
}

fn length(d:[Point;2]) -> f64 {
	let dx = d[0].x - d[1].x;
	let dy = d[0].y - d[1].y;
	let dd = dx*dx + dy*dy;
	dd.sqrt()
}

struct GVector {
	p1: Point, p2: Point, 
}


fn angle3(l: [f64;3]) -> [f64;3] {
	
	let a_v1 = acos((l[0]*l[0] + l[1]*l[1] - l[2]*l[2])/(2.0*l[0]*l[1])).to_degrees();
	let a_v2 = acos((l[0]*l[0] + l[2]*l[2] - l[1]*l[1])/(2.0*l[0]*l[2])).to_degrees();
	let a_v3 = acos((l[1]*l[1] + l[2]*l[2] - l[0]*l[0])/(2.0*l[1]*l[2])).to_degrees();

	return [a_v1, a_v2, a_v3]
}

#[derive(Debug, Copy, Clone)]
struct Triangle {
   name:[char;2],
   p : [Point;3],
}

#[derive(Debug, Copy, Clone)]
struct Tri_info{
	name: char,
	length: [f64;3],
	angle: [f64;3],
	area: f64
}

fn area(l: [f64;3]) -> f64{
	let s = (l[0]+l[1]+l[2])/2.0;
	let a = s*(s-l[0])*(s-l[1])*(s-l[2]);

	return a
}

/****************** Add to this function *************************/
fn check_triangle(t:Triangle) {
	println!("\nMytriangle\n{:?}\n",t);

	let triangle_vec = [[t.p[0], t.p[1]], [t.p[1], t.p[2]], [t.p[2], t.p[0]]];

	let triangle_lengths = [length(triangle_vec[0]), length(triangle_vec[1]), length(triangle_vec[2])];

	// find length of each vector in triangle

	let mut _n = 0;
	for i in triangle_lengths.iter() {
		_n += 1;
		println!("Side {}:\nLength: {:?}", _n, i);
	}

	// find angle of each point in triangle
	println!();

	let mut _n = 0;
	for i in angle3(triangle_lengths).iter() {
		_n += 1;
		println!("Side {}:\nAngle: {:?}°", _n, i);
	}

	// find area of triangle
	println!();
	println!("Area: {:?}\n", area(triangle_lengths));

	println!("================================================================");




}

/****************** Read .. and understand later  *************************/
fn read_triangles( s:String ) -> u64 {
	// print!("File contains:\n{}\n",  s);
        let lines:Vec<&str> = s.split('\n').collect();
	let n_lines = lines.len();
	// println!("s has {} lines", n_lines );
	let mut n_tri:u64 = 0;
	for j in 0..n_lines {
		let t_set:Vec<&str> = lines[j].split(' ').collect();
		let n_tokens = t_set.len();
		if n_tokens < 6 { break; }
		// println!("t_set[{}] has {} tokens", j, n_tokens );
		for k in 0..n_tokens { print!(" {}:{}", k, t_set[k]); }
		// println!();
		let label = t_set[0];
		// print!("T {}: ", label);
		let mut k1:usize = 1;
		let mut ps = [Point::new();3];
		for m in 0..3 { 
			// print!("m = {} k1 {} {},{}, ", m, k1, t_set[k1], t_set[k1+1] );
			let x: f64 = t_set[k1].parse().unwrap(); 
			k1 = k1 + 1;
			let y: f64 = t_set[k1].parse().unwrap(); 
			// print!("Point ({},{}) ", x, y );
			k1 = k1 + 1;
			// let p = Point{x:x,y:y};
			ps[m] = Point{x:x,y:y};
			}
		let namej = ['A','A'];
		let tj = Triangle{name:namej,p:ps };
		check_triangle(tj);
		n_tri += 1;
		// println!();
	}
	n_tri
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("triangles.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let mut n_t:u64 = 0;
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        // Ok(_) => print!("{} contains:\n{}", display, s),
        Ok(_) => { n_t = read_triangles(s); }
    }
    println!("{} triangles read", n_t);
    // `file` goes out of scope, and the "hello.txt" file gets closed
}
