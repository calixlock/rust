
/// document
// module 사용    
use std::env; 
pub fn _003_() {
    println!("OS: {}", env::consts::OS);
    println!("ARCH: {}", env::consts::ARCH);

    // integers
    /*  9.0 != interger >> f32 / f64
    
    + plusSign 
    - minusSing
    bits = 8bit = 1byte 

    i8, i16, i32, i64, i128 and isize > sined integer > +,- 둘다 존재
    u8, u16, u32, u64, u128 and usize > unsigned integer > +만 존재
    
    isize > computer architecture에 따른 32bit || 64bits 자동 지정
    
    isize
    포인터 크기의 부호 있는 정수 타입
    메모리의 어떤 위치를 참조하는 데 필요한 바이트 수
    포인터 크기에 따라 달라지며, 메모리의 어떤 위치든 참조할 수 있습니다.
    음수, 0, 양수를 표현할 수 있습니다.
    주로 메모리 주소, 위치, 인덱스, 길이 등을 다룰 때 사용됩니다.

    32bits 운영체제 > isize == i32 > 32bits > 4bytes
    64bits 운영체제 > isize == i64 > 64bits > 8bytes
    */
// 변수를 선언 했으나 사용하지 않을경우 _t1으로 warning 방지 가능
    let _t1 = 100; // isize를 지정 안할시 일반적으로 i32로 셋팅
    let _t2 : isize = 100; //
    let _t3 : i8 = 100;
    // i8 : 8비트(1바이트) 정수 타입 지정 2^8 = 256 
    // -128 ~ 127까지 가능

    let n1 :u8 = 100; // 255
    let n2 = 50; // i64
    let n3 = n1 + n2;
    /*
    다른 타입끼리 연산 안됨
    u8 + u16
    i8 + i8
    */
    println!("n1 = {} / n3 = {}",n1,n3);

}