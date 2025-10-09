use std::io;
//use std::str::FromStr;
use rand::Rng;

fn main() -> io::Result<()> {
    let num: i32 = rand::rng().random_range(1..=100);
    let mut palpite = String::new();
    
    loop {
        palpite.clear();
        println!("Digite seu palpite: ");
        io::stdin().read_line(&mut palpite)?;
        
        match palpite.trim().parse::<i32>() {
            Ok(num_palpite) => {
                if num_palpite > num {
                    println!("O número é menor");
                } else if num_palpite < num {
                    println!("O número é maior");
                } else {
                    println!("Parabéns, você acertou!");
                    return Ok(());
                }
            }
            Err(_) => {
                println!("Por favor, digite um número válido!");
                continue;
            }
        }
    }
}