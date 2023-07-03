fn main() {

    /*variables declaration in rust */
    let _num1: i32 =9;
    let _num2: f32 =9.2;

    //use the word mut if you want to reassign the variable

    let mut _num3: i32 = 3;

    _num3 = 8;

    print!( "{} {} {} ",_num1, _num2, _num3);


    /*immutable literal stored in read-pn;y data */

    let name1: &'static str = "\nEgineer kash"; //this string can not be modified only read

    println!("{} ", name1);

    /*create mutable growable string on heap */
    let mut name2: String = String::new();
    name2 = name2 + "enginer";
    name2.push_str(" man");

    // println!("{} ", name2);

    /* string slices: an immutable portion of the string */

    let name3: &str = &name1[..8];
    let name4: &str = &name1[9..];

    
    println!("{} {}", name3, name4);

    /*loop over characters */



for c in name2.chars(){
    println!("{}", c);
}


}
