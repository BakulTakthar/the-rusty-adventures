use std::io; // prelude is the set of functions and accesbilities that are emmployed in every program by default in rust
use rand::Rng;             // for the functions or commands that are not present in the prelude we use "use" (pun intended)
use std::cmp::Ordering;

// TODO: implement the maximum tries system of 5 tries and comp and usr scores
fn main(){
    println!("the guessing game is on baby!\nyou will get a maximum of 5 tries\nmax points 5");
    loop {
        println!("drop your guess >>");

        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read");

        if input.trim() == "quit" {
                println!("you have decided to chicken out huh!\nits alright though come back another time!");
                break;
            }

        let guess: u32 =  match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("your input was invalid lets try again!!");
                continue;
            },
        };
            
        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("\nYou win!\n\n");
                break;
            },
        }

        println!("the random secret number was {secret_number}\n");


        println!("you guessed {guess}\n");
    }
}
