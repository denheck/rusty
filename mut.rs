fn main() {
    // variables are immutable by default
    let limit = 10;
    // "mut" makes a variable mutable
    let mut count = 0i;

    while count < limit {
        println!("The count is {}", count);
        count += 1;
    }
}
