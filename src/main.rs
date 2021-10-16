mod practices;

use practices::*;

fn main() {
    let x = p01_anyhow::run().unwrap();
    println!("{}", x);

    p02_struct_string_field::run();
}
