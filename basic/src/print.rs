pub fn run() {
    // Print to console
    println!("Hello, world!");

    // Basic Formatting
    println!("My name is {}, I am from {}", "sics", "China");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "sics", "China", "code"
    );

    // Name Arguments
    println!(
        "{name} is from {address} and likes to {hobby}",
        name = "sics",
        hobby = "code",
        address = "China",
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);

}

#[test]
fn test() {
    run()
}