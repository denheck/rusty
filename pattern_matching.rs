// sort of like a switch statement with super powers
fn message(i: int) {
    match i {
        1 => println!("ONE!"),
        2 => println!("Two is a prime"),
        3 => println!("THREE!"),
        // leaving off the _ will result in
        // error: non-exhaustive patterns: `_` not covered
        _ => println!("no idea what that is, boss")
    }
}

fn main() {
    message(1);
    message(2);
    message(3);
}
