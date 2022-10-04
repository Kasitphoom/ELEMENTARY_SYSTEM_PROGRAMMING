use rand::{Rng};

fn lut_sin(x:f64) -> (f64, f64, f64){
    const N:usize = 92;
    let mut lut:[f64;N] = [0.0;N];
    for i in 0..N {
        lut[i] = ((i as f64).to_radians()).sin();
    }

    let x_round = x.round() as i32;
    let x_next = (x_round + 1) as i32;
    let x0 = lut[x_round as usize];
    let x1 = lut[x_next as usize];
    let res = x0 + (x1 - x0) * (x - x_round as f64)/(x_next as f64 - x_round as f64);
    let real = (x.to_radians()).sin();
    let diff = (res - real).abs();
    
    return(res as f64, real, diff)
}

fn main() {
    
    let test_lut:[f64;10] = [0.0, 11.0, 20.302, 35.0023, 49.0, 57.5, 60.0, 78.9, 89.111, 90.0];
    println!("==========================================================");
    println!("Sin value \tLook up value \t\tComputed \t\tDifferences");
    let mut avg = 0.0;
    for i in 0..10 {
        let x = test_lut[i];
        let (res, real, diff) = lut_sin(x);
        println!("sin({})   \t{:.16} \t{:.16} \t{:.16}", x, res, real, diff);
        avg += diff;
    }
    avg = avg/10.0;
    println!("Average difference: {:.16}", avg);
    println!("==========================================================");

    println!("==========================================================");
    println!("Cos value \tLook up value \t\tComputed \t\tDifferences");
    let mut avg = 0.0;
    for i in 0..10 {
        let x = test_lut[i];
        let (res, real, diff) = lut_sin(90.0 - x);
        println!("cos({})   \t{:.16} \t{:.16} \t{:.16}", x, res, real, diff);
        avg += diff;
    }
    avg = avg/10.0;
    println!("Average difference: {:.16}", avg);
    println!("==========================================================");
    
    // const N:usize = 101;
    // let mut test_lut2 = [0.0; N];
    // for i in 0..N{
    //     test_lut2[i] = rand::thread_rng().gen_range(0.0..=90.0);
    // }
    // println!("==========================================================");
    // println!("Sin value \t\t\tLook up value \t\tComputed \t\tDifferences");
    // let mut avg = 0.0;
    // for i in 0..N {
    //     let x = test_lut2[i];
    //     let (res, real, diff) = lut_sin(x);
    //     println!("sin({})   \t{:.16} \t{:.16} \t{:.16}", x, res, real, diff);
    //     avg += diff;
    // }
    // avg = avg/10.0;
    // println!("Average difference: {:.16}", avg);
    // println!("==========================================================");
}
