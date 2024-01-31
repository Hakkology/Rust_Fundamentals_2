fn main(){

    for x in 1..11 {
        
        if x==7 {
            break;
        }
        if x==5 {
            continue;
        }
        println!("x is {}.", x);
    }

    let mut sum = 0;
    for x in 1..11{
        if x%2 == 0{
            sum += x*3;
        } else {
            sum += x*7;
        }
    }
    println!("total sum is {}.", sum);

}