// Rust has a test framework built in
// run with "rustc --test testing.rs"

// test methods are only present in the executable if
// compiled with the --test flag

#[test]
fn test_method() {
    // this takes no arguments and returns nothing
    // errors indicate test failure
    println!("");
}

#[test]
fn failing_test(){
    fail!("Test Failed");
}
