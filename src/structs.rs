// Structs - used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

// Struct with functions
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        return Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        };
    }

    // Get full name
    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        return (self.first_name, self.last_name);
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut tuple_c = TupleColor(255, 0, 0);
    tuple_c.0 = 200;

    println!("Tuple Color: {} {} {}", tuple_c.0, tuple_c.1, tuple_c.2);

    let mut p = Person::new("Mary", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);

    p.set_last_name("Williams");

    println!("Full Name: {}", p.full_name());

    println!("Person Tuple: {:?}", p.to_tuple());

    println!("-----------------------");
}
