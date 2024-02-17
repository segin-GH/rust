fn main() {
    println!("Hello, world!");
    assert_eq!(foo(), 3);
}

fn foo() -> i32 {
    return 2;
}
