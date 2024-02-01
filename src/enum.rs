#[derive(Debug)]
enum GenderCategory{
    Male,
    Female
}

enum AgeState{
    Available,
    Barelyavailable,
    Unavailable
}

enum Kamp{
    Ders(String, i32),
    Hoca([String; 2])
}

// #[derive(Debug)]
// enum PersonCategory{
//     Name(String),
//     Count(i32)
// }

// enum Option<T>{
//     Some(T), 
//     None,
// }


enum CarType{
    Hatch,
    Sedan,
    SUV
}

fn main(){

    let male = GenderCategory::Male;
    let female = GenderCategory::Female;

    println!("{:?}", male);
    println!("{:?}", female);

    // let is_even = is_even(5);
    // println!("{:?}", is_even);

    print_size(CarType::SUV);
    print_size(CarType::Sedan);
    print_size(CarType::Hatch);

    match is_even(6){
        Some(data) => {
            if data{
                println!("Even no");
            }
        },
        None => {
            println!("not even");
        }
    }

    age_status_command(age_status_handler(22));

    // let p1: PersonCategory = PersonCategory::Name(String::from("Hakan"));
    // let p2: PersonCategory = PersonCategory::Count(18);


    let ders1 : Kamp = Kamp::Ders("Rust101".to_string(), 5);
    let hoca1 : Kamp = Kamp::Hoca(["Veli".to_string(), "Aydın".to_string()])  ;

    let ders2 : Kamp = Kamp::Ders("Rust ile Blockchain".to_string(), 6);
    let hoca2 : Kamp = Kamp::Hoca(["Emin".to_string(), "Muhammed".to_string()])  ;

    print_ders_detaylar(&ders1, &hoca1);
    print_ders_detaylar(&ders2, &hoca2);

    // match ders1 {
    //     Camp::Ders(name: String, count: i32) => {

    //     }
    // }

}


fn is_even(no: i32) -> Option<bool>{
    if no%2 == 0 {
        Some(true)
    } else {
        None
    }
}

fn print_ders_detaylar (ders: &Kamp, hoca: &Kamp){

    if let Kamp::Ders(ders_adi, ders_no) = ders {
        if let Kamp::Hoca(hoca_isimleri) = hoca {
            println!(
                "Verilen {} nolu dersin ismi '{}', hocası {} ve {} şeklindedir.",
                ders_no, ders_adi, hoca_isimleri[0], hoca_isimleri[1]
            );
        } else {
            println!("Hoca bilgisi doğru formatta değil.");
        }
    } else {
        println!("Ders bilgisi doğru formatta değil.");
    }
}

fn age_status_handler (no: i32) -> AgeState{
    if no<18 {
        AgeState::Unavailable
    } else if no == 18 {
        AgeState::Barelyavailable
    } else {
        AgeState::Available
    }
}

fn age_status_command (age:AgeState){
    match age {
        AgeState::Unavailable => println!("Unable to vote."),
        AgeState::Barelyavailable => println!("Can vote starting this year."),
        AgeState::Available => println!("Available for vote.",),
    }

}

fn print_size (car: CarType){
    match car {
        CarType::Hatch => println!("Small sized car"),
        CarType::Sedan => println!("Medium sized car"),
        CarType::SUV => println!("Large sized car"),
    }
}

