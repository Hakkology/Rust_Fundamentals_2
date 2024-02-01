use chrono::{NaiveDate, Datelike};
//to be corrected.

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

    //let ogrenci_array: [&Ogrenci;3] = [&ogrenci, &ogrenci2, &ogrenci3];
    let mut ogrenci_vec: Vec<Ogrenci> = vec![ogrenci, ogrenci2, ogrenci3];

    let student_var = add_student(String::from("Veli"),String::from("Rust"), 23, NaiveDate::from_ymd_opt(2000, 11, 07).expect("Invalid birthdate"), &mut ogrenci_vec);
    let student_var2 = add_student(String::from("Mustafa"),String::from("Rust"), 35, NaiveDate::from_ymd_opt(1988, 10, 07).expect("Invalid birthdate"), &mut ogrenci_vec);
    let student_var3 = add_student(String::from("Melek"),String::from("Rust"), 28, NaiveDate::from_ymd_opt(1995, 4, 07).expect("Invalid birthdate"), &mut ogrenci_vec);

    // print_student_details(ogrenci);
    // print_student_details(ogrenci2);
    // print_student_details(ogrenci3);

    // println!("{}", average_student_age(&ogrenci_array).to_string());
    // print_student_details(&ogrenci_array);

}

fn print_student_details(ogrenci: &[&Ogrenci;3]){

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

fn print_student_details_from_vec(ogrenci: &mut &Ogrenci){

    println!(
        "Adı: {}, Departmanı: {}, Yaşı: {}, Doğum Tarihi: {}-{}-{}",
        ogrenci.name,
        ogrenci.department,
        ogrenci.age,
        ogrenci.birthdate.year(),
        ogrenci.birthdate.month(),
        ogrenci.birthdate.day()
    );
}

fn average_student_age(ogrenci: &[&Ogrenci;3]) -> i32{

    let mut sum = 0;
    let mut count =0;

    for item in ogrenci {
        sum += item.age;
        count += 1;
    }

    return sum/count;

}

fn students_check(ogrenci_listesi: &mut Vec<&Ogrenci>){

    for student in ogrenci_listesi {
        if student.age <35 && student.age >25 {
            print_student_details_from_vec(student)
        }
    }
}

fn add_student(name: String, department: String, age: i32, birthyear: NaiveDate, ogrenci_listesi: &mut Vec<Ogrenci>){
    
    let student = Ogrenci {
        name : name,
        department: department,
        age: age,
        birthdate : birthyear,
    };

    ogrenci_listesi.push(student);
}