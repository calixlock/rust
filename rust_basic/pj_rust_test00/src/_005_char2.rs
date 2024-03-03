use std::mem::size_of;
pub fn _005_() {
  println!("size of a char {} bytes", size_of::<char>());
  println!("size of a String {} bytes", size_of::<String>());
  
  // .len() > rust에서는 size of the string in bytes를 의미
  // 글자수가 아닌 바이트 수
  // u8 = 8 bits = 1 byte 
  // 아스키코드 값에 해당 하면 1byte를 사용하지만 그 이외의 경우는 더 커질 수 있다.
  println!("b.len() = {} bytes", "b".len()); // 1 byte
  let name = "kimxxx";
  println!("name.len() = {} bytes", name.len());
  let number = "1234567";
  println!("name.len() = {} bytes", number.len());
  println!("Size of string containing 'ß': {}", "ß".len());
  println!("Size of string containing '国': {}", "国".len());
  println!("Size of string containing '𓅱': {}", "𓅱".len());
  println!("--------------------------------------------------------");

  let slice = "Hello!";
  let slice2 = "안녕!"; // Korean for "hi"
  // size length bytes
  println!("Hello! is {} bytes.", slice.len());
  println!("안녕! is {} bytes.", slice2.len());
  
  // chars count
  println!("Hello! is {} bytes and also {} characters.", slice.len(), slice.chars().count());
  println!("안녕! is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}