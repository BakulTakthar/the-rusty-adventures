use std::io; // prelude is the set of functions and accesbilities that are emmployed in every program by default in rust
use rand::Rng;             // for the functions or commands that are not present in the prelude we use "use" (pun intended)
use std::cmp::Ordering;


fn main(){
    println!("the guessing game is on baby!");
    println!("drop your guess >>");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read");

    
    println!("the random secret number was {secret_number}");


    println!("you guessed {guess}");
}