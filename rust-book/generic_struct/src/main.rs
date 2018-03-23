
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: "5", y: "10" };
    //    let p3 = Point { x: 5, y: 10.0 };  // compile error
    let p4 = Point2 { x: 5, y: 10.0 };
}
