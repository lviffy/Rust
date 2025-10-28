use std::io;
use rand::{Rng, rng};
use std::cmp::Ordering;

fn main(){

    println!("Guess the Number!");
    let secret_number = rng().random_range(1..=100);
    println!("Secret Number = {secret_number}");
loop{

    println!("Enter your Guess?");
    let mut guess = String::new(); //append make sure inside loop

    io::stdin().read_line(&mut guess)
        .expect("could not read input");

    let guess : u32 =match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("please enter a valid input");
            continue;
        }
    }; //shadowing previous variable after parsing
    
   match guess.cmp(&secret_number){     //all possible cases have to cover in match
        Ordering::Equal => {
            println!("you won!");
            break;
        },
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!")
   }
} 
}