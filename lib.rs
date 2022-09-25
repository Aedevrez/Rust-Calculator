use std::io::{self};

pub fn get_calc_method() -> &'static str {
    loop {

        println!("Hangi işlemi yapmak istersiniz?\n + - * /");

        let mut islem = String::new();

        io::stdin()
            .read_line(&mut islem)
            .expect("Failed to read line");

        let islem = match islem.trim() {
            "+" => "+",
            "-" => "-",
            "*" => "*",
            "/" => "/",
            _ => {println! ("Lütfen geçerli bir işlem giriniz!"); continue},
        };
        return islem;
    }
}

pub fn get_number() -> i32 {
    loop {
        let mut sayi = String::new();
        io::stdin()
            .read_line(&mut sayi)
            .expect("Failed to read line");
        let sayi = match sayi.trim().parse() {
            Ok(n) => n,
            Err(_) => {println!("Lütfen geçerli bir sayı girin!"); continue},
        };
        return sayi;
    }
}
