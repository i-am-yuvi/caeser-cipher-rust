fn main() {
    let shift: u8 = 27;
    let mut temp_num: u8;
    let mut temp: char;
    let mut str = String::from("spongebob");
    let mut updated_str = String::new();

    for c in str.chars() {
        temp_num = c as u8 + shift % 26;
        temp = temp_num as char;

        updated_str += &temp.to_string();
    }

    println!("Updated String is {:?}", updated_str);
}
