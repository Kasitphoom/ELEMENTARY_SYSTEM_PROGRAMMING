fn main() {
    let mut _a:char = 'a';
    let mut in_pt:f64 = 0.0;
    let mut out_pt:f64 = 0.0;
    // let mut x_sum:f64 = 0.0;
    // let mut x_mean:f64 = 0.0;
    // let mut x_diff:f64 = 0.0;
    let mut _n:f64 = 0.0;
    let _z:char = '\u{7FFF}';
    
    const EPS:f64 = 1.0e-8;
    // const EPS:f64 = 0.00001;
    
    print!("Calculate Pi!\n");
    loop {
        _n += 1.0;
        out_pt = out_pt + 1.0;
        // Generate two random numbers
        let x:f64 = rand::random();
        let y:f64 = rand::random();
        // Calculate distance of (x,y) from origin
        // let d2:f64 = <your code>

        let d2:f64 = ((x * x) + (y * y)).sqrt();
        // Is this point inside the unit circle?
        if d2 <= 1.0 { 
            // <your code>
            in_pt = in_pt + 1.0;
        }
       // Calculate estimate of pi every 10000 trials
        if (_n % 1.0) == 0.0 {
            let pi_est = (in_pt / out_pt) * 4.0;
            let diff = core::f64::consts::PI - pi_est;
            print!("{} {} {} ", _n, x, y);
            println!(" pi = {} {}", pi_est, diff);
            // Is the difference too large?
            let out_of_range = diff > EPS;
            let ok = diff > 0.0;
            let mut abs_diff = f64::abs(diff);

            if abs_diff <= EPS{
                break
            }
            // print!("{} {} {}\n", out_of_range, ok, abs_diff);
            // abs_diff = diff.abs();
            // print!("{} {} {}\n", out_of_range, ok, abs_diff);
        }

    //    for exercise 2
        // x_sum += f64::abs(x);
        // x_mean = x_sum / (_n as f64);
        // x_diff = x_mean - 0.5;

        // println!("{} {}\n", x_mean, x_diff);
    }
    
    
    
}