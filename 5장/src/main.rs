use std::mem::size_of;

// char 와 string의 차이를 알아보자
fn main() {
    
    println!("Size of a char: {} bytes", size_of::<char>());
    println!("Size of string containing 'ab': {}", "ab".len());

    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());

    let slice2 = "안녕!";
    println!("Slice2 is {} bytes and also {} characters.", slice2.len(), slice2.chars().count());
}
