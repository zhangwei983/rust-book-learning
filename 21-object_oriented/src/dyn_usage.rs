use rand::Rng;

struct Sheep {}

struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooo!"
    }
}

// Return a reference to a Box containing a heap-allocated Animal.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let random_number = rand::rng().random::<f64>();
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );

    println!("--- End module: {}", module_path!());
}
