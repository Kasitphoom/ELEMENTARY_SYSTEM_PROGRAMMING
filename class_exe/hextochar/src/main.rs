fn main() {
    let hex = "073fc".to_string();
    let mut hex_vec = Vec::new();
    //str to vec
    for i in hex.chars(){
        hex_vec.push(i);
    }
    let mut dec = 0;
    let mut power = 0;
    let letter_arr = [['A', 'a'], ['B', 'b'], ['C', 'c'], ['D', 'd'], ['E', 'e'], ['F', 'f']];
    let dec_arr = [10, 11, 12, 13, 14, 15];
    for i in (0..hex_vec.len()).rev(){
        let mut num = 0;
        for j in 0..letter_arr.len(){
            if hex_vec[i] == letter_arr[j][0] || hex_vec[i] == letter_arr[j][1]{
                num = dec_arr[j];
                break
            }
        }
        if hex_vec[i].is_numeric(){
            num = hex_vec[i].to_digit(10).unwrap();
        }
        dec += num as u64 * 16_u64.pow(power);
        power += 1;
    }
    println!("{}", dec);
}
