#[derive(Debug)]
enum GenderCategory{
    Male,
    Female
}

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
            if data == true{
                println!("Even no");
            }
        },
        None => {
            println!("not even");
        }
    }
    
}


fn is_even(no: i32) -> Option<bool>{
    if no%2 == 0 {
        Some(true)
    } else {
        None
    }
}

fn print_size (car: CarType){
    match car {
        CarType::Hatch => println!("Small sized car"),
        CarType::Sedan => println!("Medium sized car"),
        CarType::SUV => println!("Large sized car"),
    }
}

