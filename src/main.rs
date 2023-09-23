use std::cmp::Ordering;
use std::io::stdin;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=10).to_string();
    let mut guess = String::new();

    for i in (1..=3).rev() {
        println!("you have {i} try");
        stdin().read_line(&mut guess).expect("some input error");
        print!("your guessed {guess} ");
        match guess.trim().cmp(&secret) {
            Ordering::Less => print!("too small"),
            Ordering::Equal => {
                print!("correct");
                return;
            },
            Ordering::Greater => print!("too big"),
        }
        guess = "".to_string();
        println!()
    }
    println!("you lose, secret number is {secret}")
}
