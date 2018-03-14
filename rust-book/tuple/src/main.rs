struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let c1 = Color(1, 2, 3);
    let p1 = Point(1, 2, 3);
    println!("{}", c1.0);
}
