use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let random_number: u32 = rand::thread_rng().gen_range(1..101);

    println!("Adivinhe o numero!");
    println!("Por favor digite sua tentativa: ");

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha em capturar o valor");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você digitou: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Você venceu!");
                break;
            }
        }
    }
}
