use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Adivinhe o número uau!");
    println!("Chute um número:");

    let random = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Nao entendi seu número");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Insira um numero");
                continue;
            }
        };

        match guess.cmp(&random) {
            Ordering::Less => println!("menor"),
            Ordering::Greater => println!("maior"),
            Ordering::Equal => {
                println!("ganhaste");
                break;
            }
        }
    }
}
