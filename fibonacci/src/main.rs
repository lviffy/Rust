use std::io;

fn main() {

loop{
    let mut n= String::new();
    println!("Enter the value of N:");
    io::stdin().read_line(&mut n).expect("Could not read!");

    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter valid number");
            continue;
        }
    };

    let mut fibonacci:usize = 0;
    let mut prev:usize = 0;
    let mut next:usize = 1;
    print!("{prev} ");
    print!("{next} ");
    for i in 0..n-2{
        fibonacci = prev + next;
        print!("{fibonacci} ");
        prev = next;
        next = fibonacci;
    }
    println!(" ");
    println!("The {n} fibonacci numbers are printed!");

}
}