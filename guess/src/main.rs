use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("secret number is {}", secret_number);

    loop 
    {
        println!("Guess a number:");
    
        // mut == mutable
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_)  => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number)
        {
            Ordering::Less      => println!("too small."),
            Ordering::Greater   => println!("too big."),
            Ordering::Equal     => 
            {
                println!("you win!");
                break;
            }
        } 
    }
}
