use colored::*;
use std::io::{self, Write};
use rand::{Rng};
use serde_json::{Value};

const HIRAGANA_MOJI: &str = include_str!("data/hiragana_moji.json");

pub fn print(colour: &str, text: &str) {
    print!("{}", format!("{}", text.color(colour).bold()));
    io::stdout().flush().unwrap();
}

pub fn println(colour: &str, text: &str) {
    println!("{}", format!("{}", text.color(colour).bold()));
}


pub fn banner() {
    println("blue", "
██╗  ██╗██╗██╗  ██╗ █████╗
██║  ██║██║██║ ██╔╝██╔══██╗
███████║██║█████╔╝ ███████║
██╔══██║██║██╔═██╗ ██╔══██║
██║  ██║██║██║  ██╗██║  ██║
╚═╝  ╚═╝╚═╝╚═╝  ╚═╝╚═╝  ╚═╝
HIRAGANA & KATAKANA");
}

pub fn coming_soon() {
    println("red", "COMING SOON!!!");
}


pub fn input_string(text: &str) -> String {
    let mut input = String::new();
    input.clear();
    print("yellow", text);
    io::stdin().read_line(&mut input).expect("failed to readline");
    input.trim().to_string()
}

pub fn input_string2(text: &str) -> String {
    let mut input = String::new();
    input.clear();
    print("yellow", text);
    io::stdin().read_line(&mut input).expect("failed to readline");
    input.trim().to_string()
}


pub fn input_i32(text: &str) -> i32 {
    let mut input = String::new();
    loop {
        print("yellow", text);
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Ok(num) = input.trim().parse::<i32>() {
            return num;
        }
        println("red","Invalid input");
    }
}

pub fn generate_random_hiragana(level: i32) -> (String, String) {
    let mut rng = rand::rng();
    let rand = match level {
        1 => rng.random_range(0..=11),
        2 => rng.random_range(12..=15),
        3 => rng.random_range(16..=26),
        4 => rng.random_range(0..=26),
        _ => rng.random_range(0..=26)
    };
    let mut rng2 = rand::rng();
    let rand2 = rng2.random_range(0..=4);
    let json: Value = serde_json::from_str(HIRAGANA_MOJI).unwrap();
    let hasil = &json["data"]["category_id"][rand]["data"];
    let moji= hasil[rand2]["moji"].as_str().unwrap_or(hasil[0]["moji"].as_str().unwrap());
    let writing= hasil[rand2]["writing"].as_str().unwrap_or(hasil[0]["writing"].as_str().unwrap());
    (moji.to_string(), writing.to_string())
}

pub fn level_hiragana() -> i32{
    loop {
        println("yellow", "\nSELECT LEVEL");
        println("blue", "[1] EASY");
        println("blue", "[2] NORMAL");
        println("blue", "[3] HARD");
        println("blue", "[4] COMBINATION ALL");
        let pilih = input_string("PILIH : ");
        return match pilih.as_str() {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            _ => continue
        }
    }

}