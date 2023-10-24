use std::io;

fn main() {

    println!("Input a nTh Fibonacci number:");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input : usize = input.trim().parse().expect("Please type a number!");

    for num in (0..input+1).rev() { 
        println!("Fib n = {num}: {}", fib(num));
    }
}

fn fib(n: usize) -> usize {
    let first_n = [0,1,1,2];

    if n <= 2 {first_n[n]}
    else { fib(n -1) + fib(n - 2) }

}
