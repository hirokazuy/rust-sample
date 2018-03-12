fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true.");
    } else {
        println!("condition was false.");
    }

    let number2 = if number < 5 {
        0
    } else {
        10
    };
    println!("The value of number2 {}", number2);
}
