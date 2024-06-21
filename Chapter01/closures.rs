fn main() {
    let doubler = |x| x * 2;
    let value = 5;
    let twice = doubler(value);
    println!("{} doubled is {}", value, twice);
    let big_closure = |x, y| {
        let z = x + y;
        z * twice
    };
    let some_number = big_closure(1, 2);
    println!("Result from closure is {}", some_number);
}