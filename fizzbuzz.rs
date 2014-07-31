#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        fail!("One is not 3");
    }
}

#[test]
fn test_div_by_three_with_three() {
    if !div_by_three(3) {
        fail!("Three should be three");
    }
}

fn div_by_three(num: int) -> bool {
    // value of the last expression gets returned
    // IMPORTANT no semicolon
    // the semicolon turns the expression into a statement,
    // and statements don’t have values
    num % 3 == 0
}

#[test]
fn test_div_by_five() {
    assert!(!div_by_five(1));
}

#[test]
fn test_div_by_five_with_five() {
    assert!(div_by_five(5));
}

fn div_by_five(num: int) -> bool {
    num % 5 == 0
}

fn main() {
    for num in range(1i, 101) {
        let answer =
            if div_by_three(num) {
                "Fizz".to_str()
            } else if div_by_five(num) {
                "Buzz".to_str()
            } else if div_by_three(num) && div_by_five(num) {
                "FizzBuzz".to_str()
            } else {
                num.to_str()
            };

        println!("{:s}", answer);
    }
}
