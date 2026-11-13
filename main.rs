use std::io;
use rand::prelude::*;

fn main()
{
    let mut num: i32 = -1;
    let mut stringynum = num.to_string();
    let mut stringyguess = "".to_string();

    loop
    {
        let mut status = "LOSE".to_string();
        
        num = rand::rng()
            .random_range(1..=100); // random number in range 1-100 inclusive
        stringynum = num.to_string();
       
        println!("Guess a number 1-100!\nYour guess:");
        io::stdin()
            .read_line(&mut stringyguess)
            .expect("I don't wanna work, too hard :p");

        stringyguess = stringyguess[0..stringyguess.len() - 1].to_string();

        if stringyguess == stringynum
        {
            println!("Number was {stringynum}\nYou guessed: {stringyguess}\nYou WIN!!!\n");
        }
        else
        {
            println!("Number was: {stringynum}\nYou guessed: {stringyguess}\nYou LOSE\n");
        }
        
        stringyguess = "".to_string();
    }
}
