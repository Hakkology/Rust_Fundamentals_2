fn main(){

    let mut s1 = String::from("Rust");
    let s2 = &s1;

    s1 = String::from("foo");
    // ownership is critical on String due to the fact that it works on Heap side of memory.

    let n1 =2015;
    let n2 =n1;

    println!("{}", n1.to_string());
    // here the operation is concluded on stack, so ownership is no concern.

    let mut s = String::from("Rust");
    do_something(&mut s);
    println!("{}", s);
    s.push_str("01");

    // provide a new int and change the value accordingly.
    let int: u32 = 32;
    let new_int = do_something_else(int);
    println!("{}", int);
    println!("Modified int: {}", new_int);

    // change the actual value through reference.
    let mut int2: u32 = 32;
    do_something_else2(&mut int2);
    println!("{}", int2); 

    // only change the print value.
    let int3: u32 = 32;
    do_something_else3(int3); 
    println!("{}", int3); 



}

fn do_something(x: &mut String) {
    println!("{}", x);
    x.push('1');
}

fn do_something_else(x: u32) -> u32 {
    println!("{}", x);
    x + 10
}

fn do_something_else2(x: &mut u32) {
    println!("{}", *x);
    *x += 10;
}

fn do_something_else3(x: u32) {
    println!("Value inside do_something_else: {}", x + 10); // Show the modified value inside the function
}