use std::io;

fn main() {

    loop{
        let mut option = String::new();
        let mut input = String::new();

        println!("Select a number");
        println!("1 to convert Fahrenheit to Celsius");
        println!("2 to convert from Celsius to Fahrenhit");
        println!("3 to exit");
        println!("Option:");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: usize = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if option == 1 {
           println!("Input Fahrenheit");

           io::stdin()
               .read_line(&mut input)
               .expect("Failed to read line");

           let input: f64 = match input.trim().parse() {
               Ok(num) => num,
               Err(_) => continue,
           };

           println!("{input} Fahrenheit = {} Celsius", 5.0 * (input - 32.0) / 9.0);

        } else if option == 2 {
           println!("Input Celsius");

           io::stdin()
               .read_line(&mut input)
               .expect("Failed to read line");

           let input: f64 = match input.trim().parse() {
               Ok(num) => num,
               Err(_) => continue,
           };

           println!("{input} Celsius = {} Fahrenheit", 32.0 + (9.0 * input) / 5.0);

        } else if option == 3 {
            break println!("Bye ;)")
        }

    } 

}
