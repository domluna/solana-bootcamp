use std::io;
use std::io::Write;

fn fizzbuzz(n: i32) {
    let mut fizzbuzz_count = 0;
    for i in 1..n + 1 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
            fizzbuzz_count += 1;
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
    println!("FizzBuzz happened {} times", fizzbuzz_count);
}

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input: i32 = input.trim().parse().unwrap();
    println!("FizzBuzz up to {}", input);
    fizzbuzz(input);
}
