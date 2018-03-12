fn while_ex() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index = index + 1;
    }
}

fn for_ex() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn countdown() {
    for number in (1..4).rev() {
        println!("number: {}", number);
    }
}

fn main() {
    while_ex();
    for_ex();
    countdown();
}
