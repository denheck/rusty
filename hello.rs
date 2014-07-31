fn main() {
    let greeting_message = "Hello?";

    // println! is a macro
    println!("{}", greeting_message);

    // parallel hello world
    for _ in range(0u, 10) {
        spawn(proc() {
            println!("{}", greeting_message);
        });
    }
}
