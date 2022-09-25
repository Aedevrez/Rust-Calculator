use calculator::*;
use std::io::{self, Write};

fn main() {
    //Read user input for calc method
    let calc_method: &str = get_calc_method();

    //Read user input again for variables
    print!("Birinci sayı:");
    io::stdout().flush().unwrap();
    let sayi_bir = get_number();

    print!("İkinci sayı:");
    io::stdout().flush().unwrap();
    let sayi_iki = get_number();
    //Commit calculation
    match calc_method {
        "+" => {println!("{}", sayi_bir + sayi_iki);},
        "-" => {println!("{}", sayi_bir - sayi_iki);},
        "*" => {println!("{}", sayi_bir * sayi_iki);},
        "/" => {println!("{}", sayi_bir as f32 / sayi_iki as f32);},
        _ => {println!("Bö");},
    };
    //Pog
    println!("This program works!");
}
