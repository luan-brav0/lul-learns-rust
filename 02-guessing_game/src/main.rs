use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    // println!("🤫 The secret number is {secret_number}");
    loop {
        println!("🟢 Please input your guess");
        let mut guess: String = String::new();
        io::stdin()
            // Methods called. Expect error and handle.
            .read_line(&mut guess)
            .expect("🔴 Failed to read line");

        // Trims whitespaces and tries to parse to desired type.
        // No error handling:
        // let guess: u32 = guess.trim().parse().expect("🔴 Please type a number!");
        // With error handling:
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("🟩 You guesses {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("🟨 Too small"),
            Ordering::Greater => println!("🟨 Too big"),
            Ordering::Equal => {
                println!("🟩 YOU WIN 🎉!");
                break;
            }
        }
    }
}
