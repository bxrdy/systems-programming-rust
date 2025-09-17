// Topic: Loop constructs for iteration patterns

// Program requirements:
// * Display "1" through "4" in the terminal

// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn loop_func(mut n: i32) {
    loop {
        println!("{:?}", n);
        n = n - 1;
        if n == 0 {
            break;
        }
    }
    println!("done!");
}

fn main() {
    loop_func(4);
}