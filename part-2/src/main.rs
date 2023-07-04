fn main() {
    println!("Hello, world!");
   printing::goodbye();
   printing::time_stuff::give_us_the_time();
   printing::time_stuff::give_us_the_time2();
}


//provides a way for us to organize our code into groups of related functionality
mod printing{

   //nestting modules help in organizing related functionality
   pub mod time_stuff;
   pub fn goodbye() {
        println!("Goodbye, world!");
    }
    
}

