pub fn _012_(){
  let num1 = 11; // i32
  println!("num1 : {}", num1);
  // & : reference
  let single_ref = &num1; // ref to my_num1 &i32
  println!("*single_ref(=&num1) : {}", *single_ref);
  let double_ref = &single_ref; // &&i32
  println!("*double_ref(=&single_ref) : {}", *double_ref);

  /*
  & 기호는 참조자(Reference)를 생성하는 데 사용됩니다. 
    참조자는 특정 값에 대한 참조, 즉 메모리 주소를 가리킵니다.
    이를 통해 원래의 값을 복사하지 않고도 그 값을 사용할 수 있습니다.

  예를 들어, 
    let num1 = 11  num 변수를 선언하고, 그 변수에 정수 11을 저장합니다. 
    let single_ref = &num1; 코드는 num1에 대한 참조자를 생성하고,
    그 참조자를 single_ref라는 변수에 저장합니다. 
    이렇게 하면 single_ref를 통해 num1의 값을 읽을 수 있습니다.
    참조자는 Rust의 소유권 시스템의 핵심 요소로, 
    데이터를 안전하게 공유하거나 변경하는 데 사용됩니다. 
    이는 메모리 안전성을 보장하며, 런타임 오류를 방지하는 데 도움 
  */

  /*
  스택(Stack): 
    스택은 LIFO(Last In, First Out) 방식으로 데이터를 저장하는 메모리 영역
    함수 호출이 발생할 때마다 스택 프레임이 생성되고, 
    이 프레임 내에는 함수의 매개변수, 지역 변수 등이 저장. 
    함수 호출이 종료되면 해당 스택 프레임은 사라지며,
    이 과정은 컴파일러에 의해 자동으로 관리.

  힙(Heap): 
    힙은 프로그램의 실행 도중에 동적으로 할당되는 메모리 영역. 
    힙에 저장된 데이터의 생명주기는 스택과 달리 명시적으로 관리. 
    Rust에서는 소유권(Ownership) 시스템을 통해 힙 메모리의 안전한 관리를 보장.

  포인터(Pointers): 
    포인터는 메모리의 특정 위치를 가리키는 변수. 
    Rust에서는 참조자(References)와 스마트 포인터(Smart Pointers) 등 여러 종류의 포인터를 제공
    참조자는 데이터를 빌려오는 역할을 하며, 
    스마트 포인터는 힙에 저장된 데이터를 소유하고 관리하는 역할.

  Rust는 이러한 메커니즘을 통해 
    메모리 누수, 
    댕글링 포인터(Dangling Pointers), 
    데이터 경쟁(Data Races) 등의 문제를 방지.
   */
  /*
  
   */
}