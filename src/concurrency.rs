use std::mem;
use std::rc::Rc;

#[derive(Debug)]
enum Kamp{
    Ders(String, i32),
    Hoca(String)
}

fn main(){

    // Thread
    // Task
    // Concurrency (Eş güdümlü)
    // Process
    // Program

    // Thread => Task => Process => Program

    // RWA
    // WAR

    // Data Hazards and its Handling Methods

    // Box <T> for allocating values on heap
    // Rc<T> reference counting type that enables multiple ownership
    // Arc<T>
    // Ref<T>

    // thread safe değil
    let box_var = Box::new(Kamp::Ders(("Rust 102").to_string(), 10));
    println!("{:?}", &box_var);
    println!("{}", mem::size_of_val(&box_var).to_string());
    //box_var.as_mut();

    let rc_var = Rc::new(Kamp::Hoca("Emin Fedar".to_string()));
    println!("{:?}", &rc_var);
    println!("{}", mem::size_of_val(&rc_var));

}