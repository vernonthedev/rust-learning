mod arrays;
mod condition;
mod funcs;
mod loops;
mod print;
mod tuples;
mod types;
mod vectors;

fn main() {
    println!("Hello, world!");
    print::run();
    println!("Number: {}", 1);
    println!("{} is from {}", "Vernon", "Miami");
    println!("{0} is from {1} and {1} is great", "Vernon", "Miami");

    println!("{} is liked by {name}", "Coding", name = "vernonthedev");

    println!(
        "{name} likes {action}",
        name = "vernonthedev",
        action = "Coding"
    );

    println!("Binary: {:b} HEX: {:x} Octal: {:o}", 10, 10, 10);

    // Debug Trait
    println!("{:?}", (12, "Hey", true));

    println!("10 + 10 = {}", 10 + 10);

    types::run_types();
    tuples::run_tuples();
    arrays::run();
    vectors::run();
    condition::run();
    loops::run();
    funcs::run();
}
