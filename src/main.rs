fn main() {
    println!("Hello, world!");

    let i8_var: i8 = 12;
    println!("{}", i8_var);

    let u8_var :u8 = 32;
    println!("{}", u8_var);

    let u16_var : u16 = 40;
    let i32_var = 30;
    let i64_var: i64 = 60;
    let u128_var: u128 = 45;
    println!("{}, {}, {}, {}", u16_var, i32_var, i64_var, u128_var);

    let decimal_256 = 256;
    let hexadecimal = 0x100;
    let octal = 0o343;
    println!("decimal: {}, hexadecimal: {}, octal: {}", decimal_256, hexadecimal, octal);

    let f_var = 2.1;
    let f_var2: f32 = 2.454;
    print!("{}, {}", f_var, f_var2);

    let bool_var = true;

    println!("{}", bool_var);

    let mut tuple1: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tuple1.0);

    tuple1.0 = 80;
    println!("{}" , tuple1.0);


    let mut tuple2: (i32, f64, char) = (4, 5.2, 'Ö');
    print_that_tuple(tuple2);
    tuple2 = (527, 783.1234, 'Y');
    print_that_tuple(tuple2);
    let tuple2 = (300, 600.24, 'H');
    print_that_tuple(tuple2);

    let a =[1, 2, 3, 4, 5];
    println!("{}", a[0]);
    // Shadowing, previous a array is no longer available.
    let a: [i64; 6] = [11, 22, 33, 44, 55, 66];
    println!("{}", a[0]);
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    println!("{}", days[6]);

    let mut a: [i32;5] = [3;5];
    println!("{}" , a[2]);
    a[2] = 7;
    println!("{}" , a[2]);

    let mut camp:&str="OYK";
    println!("{}", camp);
    camp = "OYK CAMP";
    println!("{}", camp);

    let empty_string: String = String::new();
    println!("{}", empty_string);

    let mut content_string: String = String::from("OYK");
    println!("{:?}", content_string);

    let var_i = 234423;
    println!("{:?}", var_i.to_string());
    content_string = var_i.to_string();
    println!("{:?}", content_string);
    let char_var = 'B';
    println!("{:?}", char_var);

    let var2 = "ÖYK KIŞ KAMPI".replace("ÖYK", "GÜZEL");
    println!("{:?}", var2);
    // Why "{}" needs to be added ?

    let mut var3 = var2.as_str();
    println!("{}", var3);
    var3 = "DENEME";
    println!("{}", var2);
    println!("{}", var3);
    let mut var4 = var3;
    println!("{}", var4);
    var4 = "XXX";
    println!("{} {}", var3, var4);

}

fn print_that_tuple(tuple: (i32, f64, char)){
    println!("{}, {}, {}" , tuple.0, tuple.1, tuple.2);
}
