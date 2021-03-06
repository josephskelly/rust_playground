use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println!("Hello, world!");
    // guessing_game();
    variables();
}

fn print_title(title: &str) {
    println!("-------------------------");
    println!("{}", title);
    println!("-------------------------");
}

fn guessing_game() {
    let title = "Guessing Game";
    print_title(title);
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn variables() {
    let title = "Variables";
    print_title(title);
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    //Shadowing
    let z = 5;
    let z = z + 5;
    let z = z * 2;
    println!("{}", z);

    //Tuples
    let tup = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", tup.2)


}
