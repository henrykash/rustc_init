fn main() {
    println!("Hello, world!");
    let mut primes = vec![2, 3, 5, 7, 11, 13, 17, 19];
    primes.push(23);
    println!("{:?}", primes);

    //remove the last element
    primes.remove(primes.len() - 1);

    //print the vector
    println!("{:?}", primes);
}
