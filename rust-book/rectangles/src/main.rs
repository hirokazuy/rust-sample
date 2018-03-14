
fn main() {
    let w1 = 30;
    let h1 = 50;

    println!(
        "The area of rectangles is {} square pixels.",
        area(w1, h1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
