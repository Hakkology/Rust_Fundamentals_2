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
    if(num_dec % 5 == 1){
        println!("Bu sayının kalanı 1.");
    } 
    else if num_dec % 5 == 0{
        println!("Tam bölündü.");
    }
    else{
        println!("Bu sayının kalanı {}", num_dec % 5);
    }
}