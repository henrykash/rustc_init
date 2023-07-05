fn main() {
    let x = 5;
    let y = 6;
   let answer = find_answer(x, y);
    println!("The answer is: {}", answer);
}

fn find_answer(x: i32, y: i32) -> i32 {
    let answer = x + y + 10;
    answer  
}
