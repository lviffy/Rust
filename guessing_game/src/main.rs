use std::io;

fn main(){
    println!("Guess the Number!");
    println!("Enter your Guess?");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("could not read input");

    println!("Your guess is {guess}");
}