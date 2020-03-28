use std::io;
use rand::Rng;
use std::cmp::Ordering;


pub fn run() {

    println!("Welcome to the Guessing Game!");

    let mut rand_num_generator = rand::thread_rng();
    let secret_number = rand_num_generator.gen_range(1,101);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        let input_handler = io::stdin();
        let input_result_enum = input_handler.read_line(&mut guess);
        let _input_string_length = input_result_enum.expect("Check your input!");

        println!("You guessed: {}",guess.trim());
        
        let parse_result = guess.trim().parse();
        // let guess:u32 = parse_result.expect("BOO!");
        let guess:u32 = match parse_result{
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Guess higher."),
            Ordering::Greater => println!("Guess lower."),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
        // if guess < secret_number {
        //     println!("Guess higher.");
        // } else if guess > secret_number {
        //     println!("Guess lower.")
        // } else {
        //     println!("You got it!");
        //     break;
        // }
    }
    
}
