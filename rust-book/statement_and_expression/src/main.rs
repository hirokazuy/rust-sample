fn test1() {
    let x = 6;  // OK
}

/* NG. because let is a statement. statement not return value!!
fn test2() {
    let x = (let y = 6);
}
 */

fn test3() {
    let x = 5;

    let y = {
        let x = 3;
        // not use ";", when use ";" in this line,
        // compiler parse line to statement.
        x + 1
    };

    println!("The value of y is: {}", y);
}

/**
 * function with return value.
 */
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    test1();
    //test2(); // compile error.
    test3();
    let x = five();
    println!("The return value is {}", x);

    println!("The plus one value is {}", plus_one(3));
}
