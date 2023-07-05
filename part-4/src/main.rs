fn main() {
    let x = 5;
    let y = 6;
   let answer = find_answer(x, y);
    println!("The answer is: {}", answer);
}

//functions are declared with fn keyword, and the return type is specified after an arrow ->
fn find_answer(x: i32, y: i32) -> i32 {
    let answer = x * y + 10;
    answer  
}

//writing tests in Rust is very easy, just add #[test] attribute to any function and run cargo test
#[test]
fn test_find_answer() {
    let x = 5;
    let y = 6;
    let answer = find_answer(x, y);
    assert_eq!(40, answer);
}
