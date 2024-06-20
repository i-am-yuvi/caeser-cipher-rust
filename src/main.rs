fn main() {
    let shift: u8 = 27;
    let mut temp_num: u8;
    let mut temp: char;
    let str = String::from("zzzzz");
    let mut updated_str = String::new();

    for c in str.chars() {
        if c.is_lowercase() {
            temp_num = (c as u8 - 97) + shift % 26;
            temp = (temp_num % 26 + 97) as char;
        } else {
            temp_num = (c as u8 - 65) + shift % 26;
            temp = (temp_num % 26 + 65) as char;
        }
        updated_str += &temp.to_string();
    }

    println!("Updated String is {:?}", updated_str);
}
