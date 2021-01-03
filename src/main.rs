use scicalc_rs::measurement::Measurement;
fn main() {
    let height: Measurement = Measurement::new(1.75, 0.01);
    let increment = Measurement::new(0.5, 0.001);

    println!("Height: {}", height);
    println!("Increment: {}", increment);
    println!("New height: {}", increment - height);
}
