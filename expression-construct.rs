fn main() {
    // many constructs are also expressions
    let item = "muffin";

    let price: f64 =
        if item == "salad" {
            3.50
        } else if item == "muffin" {
            2.25
        } else {
            2.00
        };

        // semicolon must go at the end to make it one unified expression

        println!("Your item costs: {}", price);

        // functions are also declarations
        fn is_four(x: int) -> bool {
            // result of expression used as return variable
            x == 4
        }

        if is_four(4) {
            println!("Don't worry 4 is still 4");
        }
}
