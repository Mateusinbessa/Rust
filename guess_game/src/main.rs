extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() 
{
    println!("Guessing the number game!!!!");

    let secret_number: i8 = rand::thread_rng().gen_range(1, 101);
    let mut count: i8 = 1;

    loop 
    {
        println!("Type a number: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i8 = match guess.trim().parse()  {
            Ok(num) => num,
            Err(_) => {
                println!("Type a number papito!");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        println!("Suas tentativas: {}", count);
        count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win tha game");
                break;
            }
        }
    }
}