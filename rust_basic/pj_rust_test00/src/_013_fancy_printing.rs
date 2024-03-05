pub fn _013_(){
  println!("c:\\drive\\dir1\\dir2");
  println!(r#"c:\drive\dir1\dir2"#);//raw text
  let num1 = &9;
  println!("{:p}",num1); // pointer 출력 메모리 위치 출력
  let num1 = 9000;
  println!("{:X}",num1); // 
}