pub fn run() {
    println!("Print from the print.rs file");

    let name = "Brother";
    let mut age = 28;

    println!("{} is {} old.", name, age);
    age = 45;
    println!("{} is {} old.", name, age);

    /**
     * Defining Constants.
     */
    const MAIN_CONSTANT: i32 = 001;
    println!("{} is my constant", MAIN_CONSTANT);

    // Assigning multiple vars.
    let (my_name, my_age) = ("Vernon", 21);
    println!("{} is now {} old.", my_name, my_age);
}
