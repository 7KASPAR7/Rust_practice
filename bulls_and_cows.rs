use rand::Rng;
use std::io;

const NUM_OF_STEPS: u32 = 10;
const DIFFICULTY: u32 = 4;

fn main() {
    let secret_number = generate_secret_number();
    println!("generated number is {}", secret_number);
    println!("The secret number is generated. Guess it!");

    for iter in 1..NUM_OF_STEPS {
        let guess_number = generate_guess_number();
        println!("guessed number is {}", guess_number);
        if compare_numbers(secret_number, guess_number) {
            println!("You won =*");
            break;
        }
        else if iter < NUM_OF_STEPS-1 {
            println!("Try again");
        }
        else {
            println!("You lost, the secret number was {}", secret_number);
        }
    }
}

fn generate_secret_number() -> u32 {

    let mut first_range: String = "1".to_string();
    let mut second_range: String = "9".to_string();

    for iter in 1..DIFFICULTY{
         first_range.push('0');
         second_range.push('9')
    }

    let secret_number = rand::thread_rng().gen_range(first_range.parse::<u32>().unwrap()..second_range.parse::<u32>().unwrap());
    secret_number
}

fn generate_guess_number() -> u32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
         Ok(num) => num,
        Err(_) => 0,
    };

    let divisor = u32::pow(10, DIFFICULTY - 1);

    if  guess / divisor >= 1 && guess / divisor < 10 {
        guess
    }
    else {
        println!("Write a correct {}-digit number instead of {}", DIFFICULTY, guess);
        generate_guess_number()
    }
}

fn compare_numbers(secret_number: u32, guess_number: u32) -> bool {

    println!("earlier secret {}", secret_number);
    let mut bulls_num: u32 = 0;
    let mut cows_num: u32 = 0;

    let mut secret_number: String = secret_number.to_string();
    let guess_number: String = guess_number.to_string();

    for iter in 0..secret_number.len() {
        if secret_number.chars().nth(iter).unwrap() == guess_number.chars().nth(iter).unwrap() {
            bulls_num += 1;
            secret_number = remove_digit(secret_number, guess_number.chars().nth(iter).unwrap())
        } 
        else if secret_number.contains(guess_number.chars().nth(iter).unwrap()) {
            cows_num += 1;
            secret_number = remove_digit(secret_number, guess_number.chars().nth(iter).unwrap())
        }
        println!("secret: {}", secret_number);
    }
    println!("bulls: {}, cows: {}", bulls_num, cows_num);
    if bulls_num == DIFFICULTY {
        true
    }
    else {
        false
    }
}

fn remove_digit(secret_number:String, digit: char) -> String {
    let mut new_secret_number = String::new();
    
    for iter in 0..secret_number.len() {
        if secret_number.chars().nth(iter).unwrap() == digit {
            new_secret_number.push('_');
        }
        else {
            new_secret_number.push(secret_number.chars().nth(iter).unwrap())
        }
    }
    new_secret_number
}