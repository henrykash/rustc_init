fn main() {
    //integer , float 
    let x = 23.2;
    let y = -10;
    let z = x * y as f64; //type casting to f64
    print!("{} * {} = {}", x, y, z);

    //boolean
    let xy_is_posstive = z > 0.;
    println!("\nIs {}  positive? {}", z, xy_is_posstive);

    //character
  let favouite_emoji = '😻';
    println!("My favouite emoji is {}", favouite_emoji);

    let favourite_bits = favouite_emoji as u32;
    println!("My favouite emoji is U+{:X} in bits", favourite_bits); //X is for hexa decimal


    //turples
    let x_and_y = (x, y);
    println!("x_and_y = {:?}", x_and_y); // ? is for debug

    let x = x_and_y.0;
    let y = x_and_y.1;

    print!("x = {}, y = {}", x, y);
 }
  