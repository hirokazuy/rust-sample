
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_original(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
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
