fn main() {
    println!("Learning control flow in Rust using finabonacci series");
       let n = 42;
    print!("Finabonacci F{} = {} ", n, fin(n));
}

//conditional statements in rust
fn fin(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut x = 1;
        let mut a = 0;
        let mut b = 1;
       
       //using while loop
        while x < n {
            let next = a + b; // next number in the series
            a = b;
            b = next;
            x += 1;
        }
        return b;
    }

//     //or we can use loop statement
//     loop {
//         let next = a + b; // next number in the series
//         a = b;
//         b = next;
//         x += 1;

//         if x < n {
//             break;
//         }
//     break
//     }
//     return b;
// }


}

