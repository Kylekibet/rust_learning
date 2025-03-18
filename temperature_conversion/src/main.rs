fn main() {
    //ask the use what to convert to
    let mut choice: String = String::new();
    println!("Enter f or c");
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice: String = choice.trim().parse().expect("expected a string");

    //get curent temp to convert
    let mut line = String::new();
    println!("Ener enter temperature :");
    std::io::stdin().read_line(&mut line).unwrap();
    let x: i32 = line.trim().parse().expect("expected a number");

    if choice == "f" {
        println!("conversion to fahrenheight");
        let y = f_to_c(x);
        println!("{}°F", y);
    } else if choice == "c" {
        println!("convertion to celcius");
        let y = c_to_f(x);
        println!("{}°C", y);
    }
}
// convert celcius to fahrenheit
fn c_to_f(mut temperature: i32) -> i32 {
    temperature *= 2;
    temperature += 30;
    temperature
}
// convert fahrenheit to celcius
fn f_to_c(mut temperature: i32) -> i32 {
    temperature -= 30;
    temperature /= 2;
    temperature
}
