use model::Hands;

mod model {

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

    impl Displayable for Fruit {
        // display() is a method of the Fruit enum type (self is the enum instance)
        fn display(&self) -> String {
            match self {
                Fruit::Apple => String::from("Apple"),
                Fruit::Banana => String::from("Banana"),
                _ => String::from("Orange"), // _ is a catch-all pattern (like default in a switch)
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
                left: Option::Some(Fruit::Apple),
                right: Option::Some(Fruit::Banana),
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
    // T is a type parameter (generic type) that implements the Displayable trait (T: Displayable)
    
        fn report_item<T: Displayable> (item: &Option<T>, which: &str) {
            match item {
                Option::Some(what) => {
                    println!("\n{} hand is holding {} ", which, what.display());
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
