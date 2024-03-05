pub fn _011_(){
  // 변수는 default : immutable  (const)
  // mut
  // mut를 통해 변환가능;
  let mut num = 1;
  println!("num : {}", num);
  num =2;
  println!("num : {}", num);

  // shadowing 같은 이름을 다시 쓰는 것 (덮어쓰기)
  let var1 = 10;
  println!("var1 : {}", var1);
  let var1 = "test"; // shadowing 적용
  println!("var1 : {}", var1);
  
  let var2 = 2;
  println!("var2_1 : {}", var2);
  {
    let var2 = "test중입니다";
    println!("var2_2 : {}", var2);
  }
  println!("var2_3 : {}", var2);
  
}