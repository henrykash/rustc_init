use model::Hands;

mod model {
    pub enum Item {
         Something(String), //string owned by the item
         Nothing
    }

    pub struct Hands {
        left: Item,
        right: Item,
    }

    impl Hands {
        pub fn new() -> Self {
            Hands {
                left: Item::Something(String::from("apple")),
                right: Item::Something(String::from("banana"))
            }
        }

        pub fn juggle(mut self) -> Hands {
            print!("Juggling ...");
            let air = self.left;
            self.left = self.right;
            self.right = air;

            return self;
        }
        pub fn report(&self) {
            Item::report_item(&self.left, "Left");
            Item::report_item(&self.right, "Right");
        }
    }

    impl Item {
        fn report_item(&self, which: &str) {
            if let Item::Something(what) = self {
                println!("\n{} hand is holding {} ", which, what);
            } else {
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
