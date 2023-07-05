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
    println!("\nx_and_y = {:?}", x_and_y); // ? is for debug

    let x = x_and_y.0;
    let y = x_and_y.1;
    print!("x = {}, y = {}", x, y);

    // Arrays 
    // what are arrays?
    // arrays are a way to define a type by giving a name to a collection of fields of other types.
    
    let x_and_y = [x, y as f64];

    println!("\nx_and_y = {:?}", x_and_y); // ? is for debug

    let x = x_and_y[0];
    let y = x_and_y[1];
    print!("x = {}, y = {}", x, y);

    //srtructs
    //what are structs?
    //structs are a way to define a type by giving a name to a collection of fields of other types.
    #[derive(Debug)]

    struct Secrets {
        x: f64,
        y: f64,
    }

    let x_and_y = Secrets { x, y };
    println!("\nx_and_y = {:?}", x_and_y); // ? is for debug
    print!("x = {}, y = {}", x_and_y.x, x_and_y.y);

    //enums
    //what are enums? 
    //enums are a way to define a type by enumerating its possible variants.

    #[allow(dead_code)]
    #[derive(Debug)]
    enum Fruits {
        Apple,
        Mango,
        Banana,
    }

    let fruit = Fruits::Mango;
    println!("\nfruit = {:?}", fruit); // ? is for debug


 }
  