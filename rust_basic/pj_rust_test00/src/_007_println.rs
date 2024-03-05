fn _007_(){
  let name = "kim";
  let age = give_age();
  println!("hi, my name is {} and my age is {}", name, age);
  println!("hi, my age is {age} and my age is {name}");
}

fn give_age() -> i32{
  //return 생략 가능 값만 적어도 된다
  return 42
}