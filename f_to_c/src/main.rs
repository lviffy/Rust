use std::io;
fn main() {

loop {
    let mut degree_in_f = String::new();

    println!("Enter a temp in F:");
    io::stdin().read_line(&mut degree_in_f).expect("could not read!");

    let degree_in_f : f64 = match degree_in_f.trim().parse(){
        Ok(num)=> num,
        Err(_) => {
            println!("please enter a valid Temperature");
            continue;
        }
    };

    let degree_in_c = f_to_c(degree_in_f);
    println!("{degree_in_f} F in celcius is {degree_in_c} C");

    break;
}
}

fn f_to_c(degree_in_f: f64) -> f64{
    (degree_in_f - 32.0)*5.0/9.0
}
