fn main() {
    // let is a "declarative statement" and doesn't return a value
    // (technically it does - the semicolon throws away the value and returns unit)
    let y = 5i;
    // an if is an expression, and returns a value
    let x = if y < 5 { "meow" } else { ":3" };
    println!("The value of x is: {}", x);
}
