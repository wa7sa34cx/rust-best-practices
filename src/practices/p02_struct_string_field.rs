#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new<N>(name: N) -> Person
    where
        N: Into<String>,
    {
        Person { name: name.into() }
    }
}

pub fn run() {
    let name = "sato";
    let person = Person::new(name);
    println!("{:?}", person);

    let name = "sato".to_string();
    let person = Person::new(name);
    println!("{:?}", person);
}
