fn lut_sin(x:f64) -> (f64, f64, f64){
    const N:usize = 10001;
    let mut lut = [[0.0,0.0];N];
    let mut m = 0.0;
    for i in 0..N {
        lut[i] = [m, ((m as f64).to_radians()).sin()];
        m += 90.0 / ((N as f64) - 1.0);
    }

    let mut top:usize = 0;
    let mut bottom:usize = 0;
    for i in 0..N {
        if lut[i][0] > x{
            top = i;
            bottom = i-1;
            break;
        }
    }

    let x0 = lut[bottom][1];
    let x1 = lut[top][1];
    let res = x0 + (x1 - x0) * (x - x.round() as f64)/((x.round() + 1.0) as f64 - x.round() as f64);
    let real = (x.to_radians()).sin();
    let diff = (res - real).abs();
    
    return(res as f64, real, diff)
}

fn main() {
    // calling fucntion for sin values
    let test_lut:[f64;10] = [0.0, 11.0, 20.302, 35.0023, 49.0, 57.5, 60.0, 78.9, 89.111, 90.0];
    println!("==================================================================================");
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
    println!("==================================================================================");





    // calling function for cos values
    println!("==================================================================================");
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
    println!("==================================================================================");
    



    
    // calling function for tan values (tan = sin/cos)
    println!("==================================================================================");
    println!("Tan value \tLook up value \t\tComputed \t\tDifferences");
    let mut avg = 0.0;
    let mut n = 0;
    for i in 0..10 {
        let x = test_lut[i];
        if x == 90.0{
            println!("Tan 90 is not defined");
            continue
        }
        let (res_sin, _real_sin, _diff_sin) = lut_sin(x);
        let (res_cos, _real_cos, _diff_cos) = lut_sin(90.0 - x);
        let res = res_sin/res_cos;
        let real = (x.to_radians()).tan();
        let diff = (res - real).abs();
        println!("tan({})   \t{:.16} \t{:.16} \t{:.16}", x, res, real, diff);
        avg += diff;
        n += 1;
    }
    avg = avg/(n as f64);
    println!("Average difference: {:.16}", avg);
    println!("==================================================================================");
}
