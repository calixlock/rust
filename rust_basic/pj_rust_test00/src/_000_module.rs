
// 모듈 선언
mod ex{
  pub mod _fn_test;
}
mod _003_integer;

// 모듈 가져오기
use ex::_fn_test::*;
use _003_integer::*;

  // 함수의 이름은 숫자로 시작하면 안됨
fn _000_() {
  _003_();
  check();
  check2();
  // _007_println();


}

fn _007_println(){
  println!("hi");
}