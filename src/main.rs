fn main() {
    println!("Hello, welcome to my balcony!");

    mod patio { // Inline module declaration
        enum Furniture {
            Chair,
            Table,
            Umbrella,
        }
    } 

    
    // mod flowers; // Uses file path src/flowers.rs

    // mod herbs; // Uses file path src/herbs/mod.rs
}
