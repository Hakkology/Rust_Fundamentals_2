use std::io;

fn main() {

    let mut input = String::new();
    let index_no: i32;

    loop{
        println!("Please enter the index for the fibonacci sequence:");
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse::<i32>(){
            Ok(val) => {
                index_no = val;
                break;
            },
            Err(_) => {
                println!("Please enter a valid integer value.");
            },
        }
    }

    println!("Fibonacci value of({}) = {}", index_no, fibonacci_value(index_no));

}

fn fibonacci_value (n: i32) -> i32 {

    if n<= 0 {
        return 0;
    }

    if n==1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n+2 {
        let next = a + b;
        a = b;
        b = next;
    }

    return b;
}