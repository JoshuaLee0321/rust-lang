use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Guess a number\n");
    loop {
        println!("Please input your guess, enter q or quit to quit");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        println!("You guessed {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(errno) => {
                println!("errno : {errno}");
                continue;},
        };
        // after insert guessing number, time to compare
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
            Ordering::Equal => {println!("you win"); return;}
        }

    }

}
