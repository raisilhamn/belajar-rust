use std::io;
use rand::Rng;


fn main() {
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);
    
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();
    // We use the let statement to create the variable. Here’s another example:
    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    // new = membuat instance kosong bertipe string 
    
    io::stdin()
        .read_line(&mut guess)
        // The & indicates that this argument is a reference, 
        // which gives you a way to let multiple parts of your code access one piece of data
        //  without needing to copy that data into memory multiple times. 
        .expect("Failed to read line");
        // Bisa juga ditulis dengan 
        // io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
    // io::stdout().flush();

}
