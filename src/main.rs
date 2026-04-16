use std::io;

const ZERO_CHAR_OFFSET: u8 = '0' as u8;
const A_CHAR_OFFSET: u8 = 'A' as u8;

fn convert(base: &u8, current_base: &u8, to_convert: &[u8]) -> String {
    let mut reconstructed: u64 = 0;
    for (index, value) in to_convert.iter().enumerate() {
        let exp = (to_convert.len() - 1 - index) as u32;
        reconstructed += (*value as u64) * (*current_base as u64).pow(exp);
    }
    
    return reconstructed.to_string();
}

fn arrayify(to_make: String) -> Vec<u8> {
    let mut arrayified = vec![];

    for c in to_make.chars() {
        let c_u8 = c as u8;

        if c_u8 - ZERO_CHAR_OFFSET < 10 {
            arrayified.push(c_u8 - ZERO_CHAR_OFFSET);
        } else {
            arrayified.push(c_u8 - A_CHAR_OFFSET + 10);
        }
    }

    return arrayified;
}

fn main() {
    loop {
        let mut base = String::new();
        let mut current_base = String::new();
        let mut to_convert = String::new();

        println!("Enter the base you want to convert to:");
        io::stdin().read_line(&mut base).expect("Failed to read line!");

        println!("Enter the base of the number you want to convert:");
        io::stdin().read_line(&mut current_base).expect("Failed to read line!");

        println!("Enter the number you want to convert:");
        io::stdin().read_line(&mut to_convert).expect("Failed to read line!");

        let base_u8 = base.parse::<u8>().expect("Failed to parse target base!");
        let current_base_u8 = current_base.parse::<u8>().expect("Failed to parse base of input number!");

        let arrayified = arrayify(to_convert.trim().to_uppercase().to_string());

        for num in &arrayified {
            if num >= &current_base_u8 {
                println!("Input base not correct!");
                continue;
            }
        }

        println!("{} in base {} converted to {} in base {}", to_convert, current_base_u8, convert(&base_u8, &current_base_u8, &arrayified), base_u8);
    }
}
