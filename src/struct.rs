use chrono::{NaiveDate, Datelike};

struct Kamp {
    field1: i32,
    field2: String,
    field3: f64,
}

struct Ogrenci {
    name: String,
    age: i32,
    department: String,
    birthdate: NaiveDate,
}

fn main() {
    let mut kamp = Kamp {
        field1: 32,
        field2: String::from("ÖYK Rust 101"),
        field3: 51.4,
    };

    println!("1: {}", kamp.field1);
    println!("2: {}", kamp.field2);
    println!("3: {}", kamp.field3);

    kamp.field1 = 45;
    println!("1: {}", kamp.field1);

    let ogrenci = Ogrenci {
        name: String::from("Hakan"),
        age: 20,
        department: String::from("Engineering"),
        birthdate: NaiveDate::from_ymd_opt(2003, 5, 23)
            .expect("Invalid birthdate"),
    };

    let ogrenci2 = Ogrenci {
        name: String::from("Kaan"),
        age: 30,
        department: String::from("Computer Science"),
        birthdate: NaiveDate::from_ymd_opt(1993, 4, 25)
            .expect("Invalid birthdate"),
    };

    let ogrenci3 = Ogrenci {
        name: String::from("Oğuzhan"),
        age: 23,
        department: String::from("Instructor"),
        birthdate: NaiveDate::from_ymd_opt(2000, 1, 08)
            .expect("Invalid birthdate"),
    };

    // print_student_details(ogrenci);
    // print_student_details(ogrenci2);
    // print_student_details(ogrenci3);

    let ogrenci_array: [Ogrenci;3] = [ogrenci, ogrenci2, ogrenci3]; 
    println!("{}", average_student_age(&ogrenci_array).to_string());
    print_student_details(&ogrenci_array);

}

fn print_student_details(ogrenci: &[Ogrenci;3]){

    for item in ogrenci {
        println!(
            "Adı: {}, Departmanı: {}, Yaşı: {}, Doğum Tarihi: {}-{}-{}",
            item.name,
            item.department,
            item.age,
            item.birthdate.year(),
            item.birthdate.month(),
            item.birthdate.day()
        );
    }
}

fn average_student_age(ogrenci: &[Ogrenci;3]) -> i32{

    let mut sum = 0;
    let mut count =0;

    for item in ogrenci {
        sum += item.age;
        count += 1;
    }

    return sum/count;

}