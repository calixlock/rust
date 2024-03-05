pub fn _009_() {
  let n1 = n1();
  // display {}  
  // basic print 정수 실수 문자열 출력
  println!("{}", n1);
  let n2 = n2();
  // debug print {:?}
  // std::fmt::Debug 벡터나 해쉬맵 출력 데이터 내부 구조 출력
  println!("{:?}",n2);
  // pretty debug print {:#?}
  // 여러줄로 출력하여 가독성 높임
}
fn n1()-> i32{
  // return 8
  7 //  return 생략가능
}
fn n2()-> () {
  //semicolon
  8; // ; 때문에 8이 아닌 ()  return되어 error 발생
  // () empty tuple, unit type (void) 
  // return 값이 없음
}
fn n2_2() {
  //  n2()와 동일하게 작동
  // 출력 결과 값이 empty tuple 상태 
  // main도 동일하게 작동
}
