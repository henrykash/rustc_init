use model::Hands;
use std::fmt::Display;

mod model {
    use std::fmt::Display;


    //trait is a collection of methods that can be implemented by any type
    pub trait Displayable {
        fn display(&self) -> String;
    }

    #[allow(dead_code)]
    enum Fruit {
        Apple,
        Banana,
        Orange,
    }

    impl Display for Fruit {

        //use a borrowed formatter object to turn the fruit object into a string
         fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
             match self {
                 Fruit::Apple => f.write_str("Apple"),
                 Fruit::Banana => f.write_str("Banana"),
                 _ => f.write_str("Orange"),
             }
         }
    }

    #[allow(dead_code)]
    // Item is a generic enum type (T is a type parameter): a pattern that can be instantiated with different types
    // pub enum Item<T> {
    //     Something(T), //string owned by the item
    //     Nothing,
    // }

    pub struct Hands {
        left: Option<Fruit>, //instanciate the Item enum with the Fruit enum
        right: Option<Fruit>,
    }

    impl Hands {
        pub fn new() -> Self {
            Hands {
                left: Some(Fruit::Apple),
                right: Some(Fruit::Banana),
            }
        }

        pub fn juggle(mut self) -> Hands {
            print!("\nJuggling ...");
            let air = self.left;
            self.left = self.right;
            self.right = air;

            return self;
        }
        pub fn report(&self) {
            report_item(&self.left, "Left");
            report_item(&self.right, "Right");
        }
    }

    // Displayable is a trait that can be implemented by any type (T is a type parameter)
    fn report_item<T: Display >(item: &Option<T>, which: &str) {
        match item {
            Some(what) => {
                println!("\n{} hand is holding {} ", which, what);
            }
            _ => {
                println!("\n{} hand is empty", which);
            }
        }
    }
}

fn main() {
    #[allow(unused_mut)]
    let mut hands: Hands = Hands::new(); // call new() with no arguments
    Hands::report(&hands); // call report() with a reference to hands

    hands = Hands::juggle(hands); // call juggle() with a mutable reference to hands

    Hands::report(&hands); // call report() with a reference to hands
}
