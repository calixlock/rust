pub fn _010_(){
  let result = fn1(3,4);
  println!("{:?}",result);

  let result2 = fn2(5,6);    
  println!("{:?}",result2);
}
fn fn1(n1:i32,n2:i32)-> i32 {
  let n3 = {
      let x = 10;
      n1 * n2 * x
  };
  n3 // return
}
fn fn2(n1:i32,n2:i32){
  let multi = n1*n2;
  println!("{:?}", multi);
}