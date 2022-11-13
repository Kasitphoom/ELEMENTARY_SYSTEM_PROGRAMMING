use std::fmt;
use std::fmt::Formatter;
use std::io;
use std::io::*;

trait Employee{
    fn get_age(&self) -> u8;
    fn get_name(&self) -> String;
}

#[derive(Debug, Copy, Clone)]
struct GPS{
    latitude: f64,
    longitude: f64,
}
const Degree: &str = "\u{00b0}";

impl fmt::Display for GPS{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut lat_direction = "";
        let mut long_direction = "";

        if self.longitude < 0.0{
            long_direction = "W";
        }
        else{
            long_direction = "E";
        }

        if self.latitude < 0.0{
            lat_direction = "S";
        }
        else{
            lat_direction = "N";
        }
        write!(f, "Latitude: {}{} {} \nLongitude: {}{} {}", self.latitude, Degree, lat_direction, self.longitude, Degree, long_direction)
    }
}

#[derive(Debug, Clone)]
struct Teacher{
    name: String,
    age: u8,
    location: GPS,
}

impl Employee for Teacher{
    fn get_age(&self) -> u8{
        self.age
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
struct Student{
    name: String,
    age: u8,
    skills: Vec<String>,
    location: GPS,
}

impl Employee for Student{
    fn get_age(&self) -> u8{
        self.age
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    
}

#[derive(Debug, Clone)]
struct Engineer{
    name: String,
    age: u8,
    location: GPS,
}

impl Employee for Engineer{
    fn get_age(&self) -> u8{
        self.age
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn read_text_line() -> String {
    let mut buffer = String::new();
    let _result = io::stdin().read_line(&mut buffer);
    buffer = buffer.trim().to_string();
    // eprintln!("Buffer read ({}) [{}]", buffer.len(), buffer );
    buffer
}

fn putinfo(structname: String, val: Vec<String>) -> (Student, Teacher, Engineer){
    eprintln!("{:?}", val);
    // empty structure prepare for data
    let mut st = Student{
        name: String::from(""),
        age: 0,
        skills: Vec::new(),
        location: GPS{
            latitude: 0.0,
            longitude: 0.0,
        },
    };

    let mut te = Teacher{
        name: String::from(""),
        age: 0,
        location: GPS{
            latitude: 0.0,
            longitude: 0.0,
        },
    };

    let mut en = Engineer{
        name: String::from(""),
        age: 0,
        location: GPS{
            latitude: 0.0,
            longitude: 0.0,
        },
    };

    // check structure name and put data

    if structname == "Student".to_string(){
        let lat_arr = val[3].split(", ").collect::<Vec<&str>>();
        st = Student{
            name: val[0].clone(),
            age: val[1].parse::<u8>().unwrap(),
            skills: val[2].split(",").map(|s| s.to_string()).collect(),
            location: GPS{
                latitude: lat_arr[0].parse::<f64>().unwrap(),
                longitude: lat_arr[1].parse::<f64>().unwrap(),
            },
        };
    }
    else if structname == "Teacher".to_string(){
        let lat_arr = val[2].split(", ").collect::<Vec<&str>>();
        te = Teacher{
            name: val[0].clone(),
            age: val[1].parse::<u8>().unwrap(),
            location: GPS{
                latitude: lat_arr[0].parse::<f64>().unwrap(),
                longitude: lat_arr[1].parse::<f64>().unwrap(),
            },
        };
    }
    else if structname == "Engineer".to_string(){
        let lat_arr = val[2].split(", ").collect::<Vec<&str>>();
        en = Engineer{
            name: val[0].clone(),
            age: val[1].parse::<u8>().unwrap(),
            location: GPS{
                latitude: lat_arr[0].parse::<f64>().unwrap(),
                longitude: lat_arr[1].parse::<f64>().unwrap(),
            },
        };
    }
    eprintln!("{} {} {}", st.name, te.name, en.name);
    return (st, te, en);

}

fn print_res(structname: String, val: Vec<String>){
    let (se, te, en) = putinfo(structname, val.clone());
    eprintln!("{} {} {}", se.name, te.name, en.name);
    if se.name != "".to_string(){
        println!("Student: {}", se.get_name());
        println!("Age: {}", se.get_age());
        println!("Skills: {:?}", se.skills);
        println!("{}\n", se.location);
    }
    else if te.name != "".to_string(){
        println!("Teacher: {}", te.get_name());
        println!("Age: {}", te.get_age());
        println!("{}\n", te.location);
    }
    else if en.name != "".to_string(){
        println!("Engineer: {}", en.get_name());
        println!("Age: {}", en.get_age());
        println!("{}\n", en.location);
    }
}

fn main() {
    let mut vals = Vec::new();
    let mut itter = 0;
    let mut struct_name = String::new();
    // create empty variable
    loop {
        
        let mut text = read_text_line();
        
        if text.len() < 3 { break; }
        else {
            let nc = text.split(": ");
            let str_vec = nc.collect::<Vec<&str>>();
            

            if str_vec[0].to_string() == "Role".to_string(){
                if itter != 0{
                    eprintln!("{}", struct_name);
                    print_res(struct_name, vals[1..vals.len()].to_vec());
                    vals.clear();
                }
                struct_name = str_vec[1].to_string();
            }
            eprintln!("{}", itter);

            vals.push(str_vec[1].to_string());
            eprintln!("Vals => {:?}", vals[1..vals.len()].to_vec());
        }
        itter += 1;
    }
    print_res(struct_name, vals[1..vals.len()].to_vec());
}
