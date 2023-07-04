fn main() {
    println!("Hello, world!");
   printing::goodbye();
   printing::announce_time();
   printing::give_us_the_time2();
}


//modules:: provides a way for us to organize our code into groups of related functionality
mod printing{

   //nestting modules help in organizing related functionality
   pub mod time_stuff;
   pub use time_stuff::give_us_the_time as announce_time; //renaming functions when importing them
    pub use time_stuff::give_us_the_time2;


   pub fn goodbye() {
        println!("Goodbye, world!");
    }
    
}

