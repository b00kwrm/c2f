use std::io;

fn main() {
    println!("How many degrees Celsius?");

    let mut celsius_degrees = String::new();

    io::stdin().read_line(&mut celsius_degrees)
        .expect("failed to read line");

    let celsius_degrees: u32 = celsius_degrees.trim().parse().expect("please type a number");

        
    let f = (celsius_degrees * 9/5) + 32;
    println!("{} degrees Celsius is {} degree Farenheit", celsius_degrees, f);
}
