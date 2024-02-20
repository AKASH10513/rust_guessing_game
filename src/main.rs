use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main()
{
    println!("Let's play the Guess  Number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=500);
 
    println!("secret_number is : {secret_number}");

    
    
    loop{
    println!("Please input Your guess");
    let mut guess =  String::new();
    


    io::stdin()
     .read_line(&mut guess)
     .expect("Failed to read Line");

    let guess: u32 =match guess.trim().parse()
    {
        Ok(num)=>num,
        Err(_) =>
        {
            
            continue,
        };
    };




    println!("You guessed :{guess}");

    

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater=>println!("Too big !!!"),
        Ordering::Equal =>
        {
             println!("You Win");
             break;
        }
    }
}

}