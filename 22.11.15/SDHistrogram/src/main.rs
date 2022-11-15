use std::io;

#[derive(Debug, Clone, Copy)]
struct GPS {
    latitude: f64,
    longitude: f64,
}

fn read_text_line() -> String {
    let mut buffer = String::new();
    let _result = io::stdin().read_line(&mut buffer);
    buffer = buffer.trim().to_string();
    // eprintln!("Buffer read ({}) [{}]", buffer.len(), buffer );
    buffer
}

fn find_mean_lat_long( lat_long: &Vec<GPS> ) -> (f64, f64) {
    let mut sum_lat = 0.0;
    let mut sum_long = 0.0;
    for gps in lat_long {
        sum_lat += gps.latitude;
        sum_long += gps.longitude;
    }
    (sum_lat / lat_long.len() as f64, sum_long / lat_long.len() as f64)
}

fn standard_deviation( lat_long: &Vec<GPS> ) -> (f64, f64) {
    let (mean_lat, mean_long) = find_mean_lat_long( lat_long );
    let mut sum_lat = 0.0;
    let mut sum_long = 0.0;
    for gps in lat_long {
        sum_lat += (gps.latitude - mean_lat).powi(2);
        sum_long += (gps.longitude - mean_long).powi(2);
    }
    ((sum_lat / lat_long.len() as f64).sqrt(), (sum_long / (lat_long.len() as f64)).sqrt())
}

fn min_max( lat_long: &Vec<GPS> ) -> (GPS, GPS) {
    let mut min_lat = 90.0;
    let mut min_long = 180.0;
    let mut max_lat = -90.0;
    let mut max_long = -180.0;
    for gps in lat_long {
        if gps.latitude < min_lat {
            min_lat = gps.latitude;
        }
        if gps.latitude > max_lat {
            max_lat = gps.latitude;
        }
        if gps.longitude < min_long {
            min_long = gps.longitude;
        }
        if gps.longitude > max_long {
            max_long = gps.longitude;
        }
    }
    (GPS{latitude: min_lat, longitude: min_long}, GPS{latitude: max_lat, longitude: max_long})
}

fn std_deciation_tometer( lat_long: &Vec<GPS> ) -> (f64, f64) {
    let (std_lat, std_long) = standard_deviation( lat_long );
    (std_lat * 111_139.0, std_long * 107_963.0)
}

fn histrogram_bins( lat_long: &Vec<GPS> ){
    let data_range = 0.00001;
    let lat_vec = lat_long.iter().map(|gps| gps.latitude).collect::<Vec<f64>>();
    let long_vec = lat_long.iter().map(|gps| gps.longitude).collect::<Vec<f64>>();
    let (min, max) = min_max( lat_long );
    // for every increment of data_range, count the number of data points
    let mut lat_count = 0;
    let mut long_count = 0;
    let mut lat_bin = min.latitude;
    let mut long_bin = min.longitude;
    println!("=======Latitude======");
    while lat_bin < max.latitude {
        for lat in &lat_vec {
            if *lat >= lat_bin && *lat < lat_bin + data_range {
                lat_count += 1;
            }
        }
        println!("{:.5} {}", lat_bin, "*".repeat(lat_count));
        lat_count = 0;
        lat_bin += data_range;
    }
    println!("=======Longitude======");
    while long_bin < max.longitude {
        for long in &long_vec {
            if *long >= long_bin && *long < long_bin + data_range {
                long_count += 1;
            }
        }
        println!("{:.5} {}", long_bin, "*".repeat(long_count));
        long_count = 0;
        long_bin += data_range;
    }
}

fn main() {
    
    let mut gps_vec = vec![];

    loop {
        let mut text = read_text_line();
        if text.len() < 3 { break; }
        else{
            text.retain(|c| !c.is_whitespace());
            let split_text = text.split(",").collect::<Vec<&str>>();

            let gps = GPS {
                latitude: split_text[0].parse::<f64>().unwrap(),
                longitude: split_text[1].parse::<f64>().unwrap(),
            };
            gps_vec.push(gps);
        }
    }
    let (mean_lat, mean_long) = find_mean_lat_long(&gps_vec);
    eprintln!("Mean Lat: {}, Mean Long: {}", mean_lat, mean_long);
    let (std_lat, std_long) = standard_deviation(&gps_vec);
    eprintln!("Standard Deviation Lat: {}, Standard Deviation Long: {}", std_lat, std_long);
    let (min_gps, max_gps) = min_max(&gps_vec);
    eprintln!("Min Lat: {}, Min Long: {}", min_gps.latitude, min_gps.longitude);
    eprintln!("Max Lat: {}, Max Long: {}", max_gps.latitude, max_gps.longitude);
    let (std_lat_meter, std_long_meter) = std_deciation_tometer(&gps_vec);
    eprintln!("Standard Deviation Lat: {:.5} meter, Standard Deviation Long: {:.5} meter", std_lat_meter, std_long_meter);
    histrogram_bins(&gps_vec);
}
