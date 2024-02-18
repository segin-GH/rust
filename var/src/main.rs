fn main() {
    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.14159;

    let age: &str = "30";

    // shadowing
    let mut age: u32 = age.trim().parse().expect("Please type a number!");

    age = age + 1;
    println!("The value of age is: {}", age);
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    println!("The value of PI is: {}", PI);
}
