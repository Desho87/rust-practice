fn main() {
    // ========== РОЗДІЛ 3: ЗМІННІ ==========
    println!("\n=== Розділ 3: Variables ===");

    // Вправа 1: Мутована змінна
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is now: {}", x);

    // Вправа 2: Shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // Вправа 3: Константи
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("Seconds in 3 hours: {}\n", THREE_HOURS_IN_SECONDS);


    // ========== РОЗДІЛ 4: БАЗОВІ ТИПИ ==========
    println!("=== Розділ 4: Basic Types ===");

    // Вправа 1: Цілі числа
    let a: i32 = 10;
    let b = 20;
    let c: i64 = 30;
    println!("Sum of {} + {} + {} = {}", a, b, c, a + b + c as i32);

    // Вправа 2: Числа з плаваючою комою
    let x_float = 2.0;
    let y: f32 = 3.0;
    println!("x / y = {:.2}", x_float / y as f64);

    // Вправа 3: Логічний тип
    let is_bigger = 10 > 5;
    let is_smaller = 10 < 5;
    println!("10 > 5 is {}, 10 < 5 is {}", is_bigger, is_smaller);

    // Вправа 4: Символьний тип
    let smiley = '😁';
    let letter = 'a';
    println!("{}{}\n", smiley, letter);


    // ========== РОЗДІЛ 6: РЯДКИ ==========
    println!("=== Розділ 6: String ===");

    // Вправа 1: Створення String
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // Вправа 2: Конкатенація
    let hello = String::from("Hello");
    let world = " world";
    let hello_world = hello + world;
    println!("{}", hello_world);

    // Вправа 3: Форматування
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let result = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", result);

    // Вправа 4: Індексування рядків (зрізи)
    let hello_cyr = "Здравствуйте";
    let slice = &hello_cyr[0..4]; // Перші 4 байти = "Зд"
    println!("First two characters: {}\n", slice);
}