fn main(){

    let mut s1 = String::from("Rust");
    let s2 = &s1;

    s1 = String::from("foo");
    // ownership is critical on String due to the fact that it works on Heap side of memory.

    let n1 =2015;
    let n2 =n1;

    println!("{}", n1.to_string());
    // here the operation is concluded on stack, so ownership is no concern.
}