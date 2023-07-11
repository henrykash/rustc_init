use std::collections::{HashMap, VecDeque, BTreeMap, HashSet};

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

    //HASHMAP: used to store key-value pairs in a hash table implementation of a map data structure 
    let mut grid = HashMap::new();
    grid.insert((2, 3), "tree");
    grid.insert((5, 7), "rock");
   
    //updating a value in a HashMap: using the entry method to check if a value exists in a HashMap and if it doesn't, it inserts it into the HashMap
    *grid.entry((3, 3)).or_insert("empty") = "bird"; //entry method returns an Entry enum which has an or_insert method that returns a mutable reference to the value at the entry if it exists or inserts the parameter as the value if it doesn't
     
     //removing a value from a HashMap
        grid.remove(&(2, 3)); //removes the value at the key (2, 3) from the HashMap

        //indexing a HashMap directly
        let coordinates = (2, 7);
        if let Some(cell) = grid.get(&coordinates) {
            println!("{}: {:?}", cell, coordinates);
        } else {
            println!("{:?} is empty", coordinates);
        }
      
    //iterating through a a HashMap using a for loop
   for (key, value) in &grid {
        println!("{:?}at {}", key, value);
    }

     // BTREEMAP: used to store key-value pairs in a B-tree implementation of a map data structure: the keys are sorted in ascending order not hash order
    let mut grid = BTreeMap::new();
    grid.insert((2, 3), "man");
    grid.insert((5, 7), "woman");

    //updating a value in a BTreeMap: using the entry method to check if a value exists in a BTreeMap and if it doesn't, it inserts it into the BTreeMap
    *grid.entry((3, 3)).or_insert("empty") = "child"; 

    //removing a value from a BTreeMap
    grid.remove(&(2, 3)); //removes the value at the key (2, 3) from the BTreeMap

    //indexing a BTreeMap directly
    let coordinates = (3, 3);   
    if let Some(cell) = grid.get(&coordinates) {
        println!("{}: {:?}", cell, coordinates);
    } else {
        println!("{:?} is empty", coordinates);
    }

    //iterating through a a BTreeMap using a for loop
    for (key, value) in &grid {
        println!("{:?}at {}", key, value);
    }


    //HASHSET 
    let mut primes = HashSet::new();
    primes.insert(2);
    primes.insert(3);
    primes.insert(5);

    //iterating through a a HashSet using a for loop
    for p in &primes {
        println!("Prime: {}", p);
    }
}
