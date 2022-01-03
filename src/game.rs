use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn game() {
    println!("Welcome to my Guessing Game!");

    // variable to generate the random number
    let secret_number = rand::thread_rng().gen_range(1..11);

    // asking for input
    println!("Please enter your guess: ");

    // creating a variable to hold the user's input
    let mut guess = String::from("");

    // using the io module(io - input output), we use the read_line method to take input from the user and passing the guess variable as a parameter.
    // the job of the read_line method is to take whatever the user types and append it to the passed argument without changing its prior contents. Hence the argument that is passed must be mutable as its contents need to be changed.
    /*
        we pass the "&" keyword before the argument which indicates that we are using a reference of the given variable. In rust a reference allows your code to access a specific 
        data without copying that data to memory multiple times and not take ownership of that data from its original owner.
     */ 
    io::stdin().read_line(&mut guess).expect("Failed to read line"); 

    // converting the guess to a number so that it can be compared to the secret number
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // printing the guess
    println!("You guessed: {}", guess);

     // printing that random number
    println!("The secret number is: {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your guess is less than the secret number, Try Again!"),
        Ordering::Equal => println!("Your guess is equal to the secret number, Congrats!"),
        Ordering::Greater => println!("Your guess is greater than the secret number, Try Again!"),
    }
}
