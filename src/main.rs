use hirakata::*;


fn main() {
    loop {
        banner();
        menu();
    }

}


pub fn menu() {
    println("yellow", "\nMENU  ");
    println("blue", "[1] HIRAGANA QUIZ");
    println("red", "[2] KATAKANA QUIZ");
    let pilih = input_i32("PILIH : ");
    match pilih {
        1 => menu_hiragana(),
        2 => coming_soon(),
        _ => {}
    }
}


fn menu_hiragana() {
    println("blue", "\n[1] MOJI QUIZ");
    println("red", "[2] KOTOBA QUIZ");
    println("red", "[3] TANGO QUIZ");
    println("red", "[4] BUN QUIZ");
    println("red", "[5] BUNSHOU QUIZ");
    let pilih = input_i32("PILIH : ");
    match pilih {
        1 => hiragana_moji_quiz(),
        2 => coming_soon(),
        3 => coming_soon(),
        4 => coming_soon(),
        5 => coming_soon(),
        _ => {}
    }
}


fn hiragana_moji_quiz() {
    let level = level_hiragana();
    let mut number = 1;
    let mut correct = 0;
    let mut wrong  = 0;
    loop {
        let (moji, writing) = generate_random_hiragana(level);
        print("yellow", &format!("[{}]", number));
        print("yellow", " Type what you see\n");
        print("purple", &format!("    {}", moji));
        let answer = input_string("  : ");
        // print("green", "    Result : ");
        if answer == writing {
            println("green", &format!("    CORRECT ({}) ", writing));
            correct += 1
        } else {
            println("red", &format!("    WRONG ({}) ", writing));
            wrong += 1
        }
    number += 1
    }

}