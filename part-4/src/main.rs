fn main() {
    let x = 5;
    let y = 6;
   let answer = find_answer(x, y);
    println!("The answer is: {}", answer);
}
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::find_answer; //import the function from the parent module or use super::* to import all items from the parent module i.e use super::*

 //tests are declared with #[test] attribute
#[test]
pub fn test_find_answer() {
    let x = 2;
    let y = 2;
    let answer = find_answer(x, y);
    assert_eq!(14, answer);
}

}


//functions are declared with fn keyword, and the return type is specified after an arrow ->
fn find_answer(x: i32, y: i32) -> i32 {
    let answer = x * y + 10;
    answer  
}



