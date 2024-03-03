use std::mem::size_of_val;

fn 006_float() {
    // _ 언더바 갯수는 여러개 상관없이 무시됨
    let n1 : u8 = 9;
    let n2 = 9u8;
    let n3 = 9_u8;
    let n4 = 100_000_000_u64;

    println!("n1 size : {} byte", std::mem::size_of_val(&n1));
    println!("n2 size : {} byte", size_of_val(&n2));
    println!("n3 size : {} byte", size_of_val(&n3));
    println!("n4 size : {} byte", size_of_val(&n4));
    
    println!("----------------------------------------------------------------");
    // float
    // f32 || f64
    // default = f64 : 8byte
    // f32 : 4 byte
    let a1 = 8.;
    let a2 = 10.3_f32;
    println!("a1 size : {} byte", size_of_val(&a1));
    println!("a2 size : {} byte", size_of_val(&a2));

    println!("n1+a1 = {}", n1 + a1 as u8);
    println!("n2+a2 = {}", n2 as f64 + a2 as f64);
    
}