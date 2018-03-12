fn while_ex() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index = index + 1;
    }
}

fn main() {
    while_ex();
}
