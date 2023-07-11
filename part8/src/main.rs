use std::collections::VecDeque;


fn main() {

    // VECTORS : vector provides dynamic array of a given type: List
    #[allow(unused_mut)]
    let mut primes = vec![2, 3, 5, 7, 11, 13, 17, 19];

 
  //iterating through a a vector using a while let loop
    let mut primes_iter = primes.iter().enumerate();
  while let Some((i, p)) = primes_iter.next() {
    println!("Into the while let\n {}: {}", i, p);

  } 

  //iterating through a vector using a for loop: which is designed to work with iterators exclusively
    for (i, p) in primes.iter().enumerate() {
        println!("Into the for {}: {}", i, p);
    }

  //VECDEQUE: used when we want to add or remove things in the front or end of a list i.e Vec
    
    let mut primes_n = VecDeque::new();
    primes_n.push_back(2);
    primes_n.push_back(3);
    primes_n.push_front(5);
      
    //iterating through a a VecDeque using a while let loop
    for (i, p) in primes_n.iter().enumerate() {
        println!("Into the VeqDeque {}: {}", i, p);
    }


    //HASHMAP: used to store key-value pairs

    

 }
