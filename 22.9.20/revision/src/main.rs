use rand::*;

fn ex1(){
    println!("\n===========EX1=============");
    let mut n = 2;
    n = n + 5;
    println!("n = {}", n);
}

fn ex2(){
    println!("\n===========EX2=============");
    let m = 10;
    let x:f64 = 1.0;
    let x10 = x * (m as f64);
    println!("x10 = {}", x10);
}

fn ex3(){
    println!("\n===========EX3=============");
    const N:usize = 10;
    let arrarN: [i32; N] = [0; N];
    println!("arrayN = {:?}", arrarN);
}

fn ex4() -> Vec<f64>{
    println!("\n===========EX4=============");
    const M:usize = 20;
    let mut vecM = Vec::with_capacity(M);
    for i in 0..M{
        vecM.push(rand::random());
    }
    println!("vecM = {:?}", vecM);
    vecM
}

fn ex5(vector: Vec<f64>){
    println!("\n===========EX5=============");
    let mut sum:f64 = 0.0;
    let mut n = 0;
    for i in 0..vector.len(){
        sum += vector[i];
        n += 1;
    }

    let avg = sum / (n as f64);
    println!("avg = {}", avg);
}

fn ex6(vector: Vec<f64>){
    println!("\n===========EX6=============");
    const N:usize = 4;
    for i in 0..vector.len(){
        if i % N == 0{
            println!("{}: {:?}", i, vector[i]);
        }
    }
}

fn ex7(){
    println!("\n===========EX7=============");
    let m = 20.0;
    let x:f64 = 1.0;
    let x10 = x * m;
    println!("{}% is {}", 1 as f64/m, x10);
}

fn ex9(){
    println!("\n===========EX9=============");
    println!("f32: \nMax: {} \nMin: {}", std::f32::MAX, std::f32::MIN);
    println!("f64: \nMax: {} \nMin: {}", std::f64::MAX, std::f64::MIN);
}

#[derive(Debug, Clone)]
struct Optimism{
    name: String,
    salary: f64,
    probability: f64,
}

impl Optimism{
    fn new(name: String, salary: f64, probability: f64) -> Self{
        Self{
            name,
            salary,
            probability,
        }
    }
}

fn ex10(){
    println!("\n===========EX10=============");
    let mut opt = Optimism{
        name: String::from("Kasitphoom Thowongs"),
        salary: 50000.0,
        probability: 0.5,
    };
    println!("Optimism: {:?}", opt);
    let opt2 = Optimism::new("Kasitphoom Thowongs".to_string(), 50000.0, 0.5);
    println!("Optimism2: {:?}", opt2);
}

#[derive(Debug, Copy, Clone)]
struct GPS{
    latitude: f64,
    longitude: f64,
}
const Degree: &str = "\u{00b0}";

fn location(gps: GPS){
    let mut lat_direction = "";
    let mut long_direction = "";

    if gps.longitude < 0.0{
        long_direction = "W";
    }
    else{
        long_direction = "E";
    }

    if gps.latitude < 0.0{
        lat_direction = "S";
    }
    else{
        lat_direction = "N";
    }
    println!("Latitude: {}{} {} \nLongitude: {}{} {}", gps.latitude, Degree, lat_direction, gps.longitude, Degree, long_direction);
}

fn ex11(){
    println!("\n===========EX11=============");

    let gps = GPS{
        latitude: 13.7506,
        longitude: 100.7943,
    };

    location(gps);
    println!("GPS: {:?}", gps);
}

fn ex12(){
    println!("\n===========EX12=============");
    let mut point: Vec<GPS> = Vec::new();
    let mut lat = 13.729; //x axis
    let mut long = 100.776; //y axis
    // 12a
    for i in 0..4{
        for i in 0..4{
            point.push(GPS{
                latitude: lat,
                longitude: long,
            });
            lat += 0.01;
        }
        long += 0.01;
    }

    // 12b
    for i in 0..4{
        print!("[ ");
        for i in 0..4{
            print!("({:.3},{:.3}) ", point[i].latitude, point[i].longitude);
        }
        print!("]");
        println!("");
    }

    // 12c

    for i in 0..point.len(){
        point.remove(0);
    }
    println!("Point: {:?}", point);
}

#[derive(Debug, Copy, Clone)]
struct Point{
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Polygon{
    points: Vec<Point>,
}
fn ex13(){
    println!("\n===========EX13=============");
    let points = vec![
        Point{x: 0.0, y: 0.0},
        Point{x: 0.0, y: 1.0},
        Point{x: 1.0, y: 1.0},
        Point{x: 1.0, y: 0.0},
    ];
    let polygon = Polygon{
        points,
    };
    println!("Polygon: {:?}", polygon);
    // 14
    println!("\n===========EX14=============");
    fn perimeter(poly: &Polygon) -> f64{
        let mut perimeter = 0.0;
        for i in 0..poly.points.len(){
            if i == poly.points.len() - 1{
                perimeter += ((poly.points[i].x - poly.points[0].x).powi(2) + (poly.points[i].y - poly.points[0].y).powi(2)).sqrt();
            }
            else{
                perimeter += ((poly.points[i].x - poly.points[i+1].x).powi(2) + (poly.points[i].y - poly.points[i+1].y).powi(2)).sqrt();
            }
        }
        perimeter
    }
    println!("Perimeter: {}", perimeter(&polygon));
    
}

#[derive(Debug, Clone)]
struct Me{
    name: String,
    age: u8,
}

impl Me{
    fn new(name: String, age: u8) -> Self{
        Self{
            name,
            age,
        }
    }
}

fn ex15(){
    println!("\n===========EX15=============");
    let name  = "กษิตภูมิ ต่อวงษ์";
    println!("Name: {}", name);

    println!("\n===========EX16=============");
    let mut me = Me{
        name: name.to_string(),
        age: 17,
    };
    println!("Me: {:?}", me);
}

fn e16(string: String){
    println!("\n===========EX16=============");
    let text = "Hello World";
    let v = text.split("");
    let vec_text = v.collect::<Vec<&str>>();
    for i in 0..vec_text.len(){
        if vec_text[i] == string{
            println!("{} is in the text at {}", string, i);
            return;
        }
    }
    println!("{} is not in the text ({})", string, -1);
}

fn main() {
    ex1();
    ex2();
    ex3();
    let vector = ex4();
    ex5(vector.clone());
    ex6(vector.clone());
    ex7();
    ex9();
    ex10();
    ex11();
    ex12();
    ex13();
    ex15();
    e16("f".to_string());
}
