fn main(){

    let num =12;

    if num%2 == 0 {
        println!("Even");
    } 
    else{
        println!("Odd");
    }

    let num = 2;
    if num >0 {
        println!("{} is positive", num);
    } else if num <0 {
        println!("{} is negative", num);
    } else{
        println!("{} is neither negative nor positive.", num);
    }

    let num_oct= 0o343;
    let num_dec = num_oct;
    if num_dec % 5 == 1{
        println!("Bu sayının kalanı 1.");
    } 
    else if num_dec % 5 == 0{
        println!("Tam bölündü.");
    }
    else{
        println!("Bu sayının kalanı {}.", num_dec % 5);
    }

    let num = 55;
    if num >200  {
        println!("Çok büyük abi.");
    }
    else if num >=99{
        println!("İdare eder be abi.");
    }
    else {
        println!("Olmadı o sayı abi.");
    }

    if num <99  {
        println!("Olmadı o sayı abi.");
    }
    else if num <200{
        println!("İdare eder be abi.");
    }
    else {
        println!("Çok büyük abi.");
    }


    let country_code = "TR";
    let code = match country_code{
        "TR" => "Turkiye",
        "US" => "Amerika",
        _ => "Bilinmeyen"
    };
    println!("State name is {:?}", code);

    let yas = 10;
    match yas {
        0..=12 => println!("Çooocuk"),
        13..=18 => println!("Ergeeen"),
        19..=30 => println!("Yetişkinn"),
        _ => println!("Fateless")
    }

    let code = 26;
    let code_str = match code {
        6 => "Baskent",
        34 => "Istanbul",
        _ => "Diger Sehirler"
    };
    println!("{:?}", code_str);
 
}