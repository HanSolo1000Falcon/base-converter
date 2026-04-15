use std::io;

const ZERO_CHAR_OFFSET: u8 = '0' as u8;
const A_CHAR_OFFSET: u8 = 'A' as u8;

/*fn convert(base: u8, current_base: u8, to_convert: &[u8]) {

}*/

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

        let arrayified = arrayify(to_convert.trim().to_uppercase().to_string());

        for thing in arrayified {
            println!("{}", thing);
        }
    }
}
