pub fn _013_(){
  // 프린트 방식 Module 참고   
  // std::fmt 
  // https://doc.rust-lang.org/std/fmt/

  println!("c:\\drive\\dir1\\dir2");
  println!(r#"c:\drive\dir1\dir2"#);//raw text
  let num1 = &9;
  println!("{:p}",num1); // pointer 출력 메모리 위치 출력
  let num1 = 9000;
  println!("num1(9000) 16진수 : {:X}",num1); // 
  println!("num1(9000) 2진수 : {:b}",num1); // 

  let title = "TODAY'S NEWS";
  println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
  let bar = "|";
  println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
  let a = "SEOUL";
  let b = "TOKYO";
  println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}