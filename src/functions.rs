fn main(){
    print(&get_pi().to_string());
    helloo();

    //ISA (infrastructure architecture) Dependent
    let mut param_no = 65;
    mutate_no_to_zero(param_no);
    println!("param_no is changed to {:?}.", param_no);
    mutate_no_to_zero_with_reff(&mut param_no);
    println!("param_no after de-referencing is {:?}.", param_no);

    let num = 4;
    println!("{}", factorial_baby(num).to_string());

}


fn get_pi() -> f64 {
    22.0/7.0
}

pub fn helloo(){
    println!("hello from helloo");
}

pub fn print(str: &str) {
    println!("{}", str);
}

fn mutate_no_to_zero(mut param_no: i32){
    param_no = param_no * 0;
    println!("param_no value is: {}.", param_no);
}

fn mutate_no_to_zero_with_reff(param_no: &mut i32){
    *param_no = 0; // de_reference
}

fn factorial_baby(num: i32) -> i32{

    let mut result = 1;
    for x in 2..=num {
        
        result *= x;
    }
    return result;
}