struct  Item {
    what: String,
    present: bool,
}

struct Hands {
    left: Item,
    right: Item ,
}

fn main() {

    let hands: Hands = Hands {
        left: Item {
            what: String::from("banana"),
            present: true,
        },
        right: Item {
            what: String::from("apple"),
            present: true,
        },
    };

    if hands.right.present {
        println!("Right hand is holding {} ", hands.right.what);
    } else {
        println!("Right hand is empty");
    }


    if hands.left.present {
        println!("Left hand is holding {} ", hands.left.what);
    } else {
        println!("Left hand is empty");
            
    }

}

