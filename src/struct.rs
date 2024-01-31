struct Kamp{

    field1: i32,
    field2: String,
    field3: f64,
}

struct Ogrenci{

    name: String,
    age: i32,
}

fn main() {

    let mut kamp = Kamp{
        field1 :32,
        field2 :String::from("ÖYK Rust 101"),
        field3 :51.4
    };

    println!("1: {}", kamp.field1);
    println!("2: {}", kamp.field2);
    println!("3: {}", kamp.field3);

    kamp.field1 = 45;

    println!("1: {}", kamp.field1);
}
