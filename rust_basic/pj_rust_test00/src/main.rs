fn main() {
    // mut : 가변 변수로 선언 / default는 immutable 상태 / 변수 타입 변경은 불가
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //char;
    let _first_letter = 'A';
    let _space = ' '; // A space inside ' ' is also a char
    let _other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let _cat_face = '😺'; // Emojis are chars too
    // 1byte = 8 bits = 2^8 (0~255)
    println!("Size of a char: {} bytes", std::mem::size_of::<char>());

    // casting = simple type change using "as"
    let n1 : u16 = 2;
    let n2 : u8 = 4;
    let n3 = n1 + n2 as u16; // casting 해서 연산이 이루어 지도록
    println!("n3 is {}", n3);

    // asciicode
    let n4  = 'a' as u8;
    println!("n4 is {}",n4);
    let n5 = 'a' as u16;
    println!("n5 is {}",n5);
}
 