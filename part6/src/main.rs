use model::{Hands, Item};

mod model {
pub struct  Item {
    what: String, //string owned by the item
    present: bool,
}

pub struct Hands {
    left: Item,
    right: Item ,
}

impl Hands {

  pub  fn new()-> Hands {
        Hands {
            left: Item {
                what: String::from("banana"),
                present: true,
            },
            right: Item {
                what: String::from("apple"),
                present: true,
            },
        }
    
    }
    

   pub fn juggle( mut hands: Hands)-> Hands{
        print!("Juggling ...");
        let air  = hands.left;
        hands.left = hands.right;
        hands.right = air;
    
        return hands;
    }
   pub fn report(hands: &Hands) {
        Item::report_item(&hands.left, "Left");
        Item::report_item(&hands.right, "Right");
    }   
    
}

impl Item {
    fn report_item(item: &Item, which: &str) {
        if item.present {
            println!("{} hand is holding {} ", which ,item.what);
        } else {
            println!("{} hand is empty", which);
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




