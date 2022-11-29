use std::time::{Duration, Instant};
use std::f64::consts::PI;
use rand::*;

const MAX_POWER: u64 = 12;
const PI2: f64 = 2.0*PI;
const BASE: u64 = 6;

fn time_diff_secs( t0:Instant ) -> f64 {
	let duration = t0.elapsed();
        let d_secs = (duration.as_secs() as f64)
           + (duration.subsec_nanos() as f64) * 1e-9;
	d_secs
}

fn empty_loop( n: u64 ) -> f64 {
  let mut sum: f64 = 0.0;
  for _j in 0..n {
    let x:f64 = (rand::random::<f64>())*PI2;
    sum += x;
    }
  let mean = sum / (n as f64);
  mean
}

fn real_loop( n: u64 ) -> f64 {
    let mut sum: f64 = 0.0;
    for _i in 0..n {
        let x:f64 = (rand::random::<f64>())*PI2;
        sum += x.sin();
    }
    let mean = sum / (n as f64);
    mean
}

// Report times in a consistent way
fn print_time( t_secs: f64, n: u64 ) {
  print!("Time: {:8.3}s, t: {:6.1}ns ", t_secs, t_secs*1.0e9/(n as f64) );
}

fn factorial( n: u64 ) -> u64 {
    let mut f: u64 = 1;
    for i in 1..n+1 {
        f *= i;
    }
    f
}

fn sin_series( x: f64, n: u64 ) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..n {
        let sign: f64 = (-1.0_f64).powi(i as i32);
        let term = sign * x.powi(2*i as i32 + 1) / (factorial(2*i+1) as f64);
        sum += term;
    }
    sum
}
fn main() {
    // println!("Accurate timing");
    // let mut n:u64 = 1;
    // for j in 1..MAX_POWER {
    //     let mut t0 = Instant::now();
    //     let mut mean = empty_loop( n );
    //     let loop_time = time_diff_secs( t0 );
    //     println!("{} {:9} ", j, n );
    //     print_time( loop_time, n );
    //     println!("m {:8.4} ", mean );
      
    //     // Now time the loop with some real code in it
    //     t0 = Instant::now();
    //     mean = real_loop( n );
    //     let loop_time = time_diff_secs( t0 );
    //     print_time( loop_time, n );
    //     println!("m {:8.4} ", mean );

    //     t0 = Instant::now();
    //     mean = sin_series( (rand::random::<f64>())*PI2, n );
    //     let loop_time = time_diff_secs( t0 );
    //     print_time( loop_time, n );
    //     println!("m {:8.4} ", mean );


    //   n = n * BASE;
    // }

    // println!("Test sin");
   
// Test your sin function for various numbers of terms
  //  for n_angles in 0..36 {
  //       println!("n_angles: {}", n_angles);
  //       let xd: f64 = 15.0 * (n_angles as f64);
  //       let x =  PI * (xd / 180.0 );
  //       println!("{:5.1} {:8.4} {:10.6}", xd, x,  x.sin() );
  //       // Now test your sin_series function for various numbers of terms
  //       let mut s = 0.0;
  //       for n_terms in 0..8 {
            
  //           s = sin_series( x, n_terms );
            
  //       }
  //       println!("{}", s);
  //   }

  println!("{}", sin_series(PI / 6.0, 20));
  println!("{}", factorial(5.0 as u64));
}
