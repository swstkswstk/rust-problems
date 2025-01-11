use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    //generate a random number
    //take user input
    //compare random number with user input
    let random_number = rand::thread_rng().gen_range(1..=100);
    let rand2 = rand::thread_rng().gen_range(1..=10);
    println!("R1 {}", random_number);
    loop {
        let mut guess = String::new();
        println!("Guess number btw 1 & 100");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i16 =match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("PLease Enter a valid number");
                continue;
            }
        };
        match guess.cmp(&random_number) {
            Ordering::Less => println!("LESS"),
            Ordering::Equal => {
                println!("EQUAL");
                break;
            }
            Ordering::Greater => println!("More"),
        }
    }
}
