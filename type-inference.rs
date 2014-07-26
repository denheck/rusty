fn main() {
    // statics must declare a type
    static MONSTER_FACTOR: f64 = 57.8;
    // local variable types can be defined optionally
    let monster_size = MONSTER_FACTOR * 10.0;
    let small_int: int = 1;

    println!("a big number: {}", monster_size);
    println!("a small number: {}", small_int);
}
