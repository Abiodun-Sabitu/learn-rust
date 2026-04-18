use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the secret number between 1 and 100!");

    // NEW WAY: In rand 0.9/0.10, use rand::random_range()
    // It's a direct, top-level function. No need to manage the generator manually.
    let secret_number = rand::random_range(1..=100);

    let mut guess = String::new();

    println!("Please input your guess.");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("You guessed: {guess} and it is less than the secret number {}!", secret_number),
        Ordering::Greater => println!("You guessed: {guess} and it is greater than the secret number {}!", secret_number),
        Ordering::Equal => println!("You win!, you guessed: {guess} and it is equal to the secret number {}!", secret_number),
    }

    
}

// fn main() {
//     println!(" guess a number between 1 and 100!"); // 2. Print a message to the user

//     let mut guess = String::new(); // 3. Create a mutable variable `guess` to store the user's input

//     io::stdin()
//         .read_line(&mut guess) // 5. Pass a mutable reference to `guess` to the `read_line` method, which will read the user's input and store it in `guess`
//         .expect("Failed to read line"); // 4. Read a line of input from the user and store it in `guess`

//     println!("You guessed: {guess}"); // 6. Print the user's guess back to them
// }
