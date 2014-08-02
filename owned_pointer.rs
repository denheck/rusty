fn main() {
    // x owns a reference to 10i
    let x = box 10i;
    // you can't make another owned pointer to this variable
    // the commented out line below will throw an error at compile time
    // error: use of partially moved value: `*x`
    //
    // let y = x;

    // this is legal because it makes a copy of the pointer and pointee
    let y = x.clone();

    println!("{:d}", *x);
    println!("{:d}", *y);
}
