mod herbs; // Uses file path src/herbs/mod.rs
mod flowers; // Uses file path src/flowers.rs
fn main() {
    #[allow(unused_variables)]

    mod patio { // Inline module declaration
        #[derive(Debug)]
        pub enum Furniture {
            Chair(String),
            Rug(String),
            Table(String),
            Umbrella(String),
        }

    }

    // using inline module does not require use statement 
    println!("Hello, welcome to my balcony!");
    let chairs = patio::Furniture::Chair(String::from("Two teal chairs."));
    dbg!(chairs); 
    let mat = patio::Furniture::Rug(String::from("Modrian welcome mat."));
    println!("Outside of the sliding door is a {:?}.", mat);
    let glass = patio::Furniture::Table(String::from("Rectangular patio table."));
    println!("Where most of the herbs are located: {:#?}", glass);
    let vinyl = patio::Furniture::Umbrella(String::from("Tan nine foot crankable umbrella."));
    println!("What keeps the balcony cool? {:#?}", vinyl);

    use flowers::Flower; // Uses file path src/flowers.rs
    let mom = Flower::JuliaChild(String::from("Yellow rose from Hunter's nursery."));
    println!("In honor of Pattie's Mom, {:?}.", mom);

    use herbs::Herbs; // Uses file path src/herbs/mod.rs
//    Basil,
//    Cilantro,
//    Dill,
//    Rosemary,

    println!("These are the herbs on my balcony:");
    let mut herb = Herbs::Basil;
    println!("{:?}", herb);
    herb = Herbs::Cilantro;
    println!("{:?}", herb);
    herb = Herbs::Dill;
    println!("{:?}", herb);
    herb = Herbs::Rosemary;
    println!("{:?}", herb);

}
