use std::io;
fn main() {

loop {
    let mut degree_in_f = String::new();
    io::stdin().read_line(&mut degree_in_f).expect("could not read!");

    let degree_in_f : f64 = match degree_in_f.trim().parse(){
        Ok(num)=> num,
        Err(_) => {
            println!("please enter a valid Temperature");
            continue;
        }
    };

    f_to_c(degree_in_f);
    break;
}
}

fn f_to_c(degree_in_f: f64) {
    let mut C : f64 = 0.0;
    C = (degree_in_f - 32.0)*5.0/9.0;

    println!("{C}");
}
