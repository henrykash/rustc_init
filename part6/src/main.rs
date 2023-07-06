use model::Hands;

mod model {
    pub struct Item {
        what: String, //string owned by the item
        present: bool,
    }

    pub struct Hands {
        left: Item,
        right: Item,
    }

   pub struct StudentDetails {
        pub name: String,
        pub age: u8,
        pub marks: u8,
        pub grade: char,
    }

    impl StudentDetails {
        
       pub fn new(name: String, age: u8, marks: u8, grade: char) -> StudentDetails {
            StudentDetails {
                name,
                age,
                marks,
                grade,
            }
        } 

        //set the student details
       pub fn set_student_details(&mut self, name: String, age: u8, marks: u8, grade: char) {
            self.name = name;
            self.age = age;
            self.marks = marks;
            self.grade = grade;
        }

        // add student details
        #[allow(dead_code)]
       pub fn add_student(name: String, age: u8, marks: u8, grade: char) -> StudentDetails {
            StudentDetails {
                name,
                age,
                marks,
                grade,
            }
        }

        //delete student details
        #[allow(dead_code)]
        pub fn delete_student(&mut self) {
            self.name = String::from("");
            self.age = 0;
            self.marks = 0;
            self.grade = ' ';
        }

        //delete a specific student details
        #[allow(dead_code)]
        pub fn delete_specific_student(&mut self, name: String) {
            if self.name == name {
                self.name = String::from("");
                self.age = 0;
                self.marks = 0;
                self.grade = ' ';
            }
        }

        //get all student details include the name, age , marks and grade
        #[allow(dead_code)]
         pub fn get_student_details(&self) -> String {
                format!(
                 "Student name is {} and age is {} and marks is {} and grade is {}",
                 self.name, self.age, self.marks, self.grade
                )
          }
        
    }

    impl Hands {
        pub fn new() -> Hands {
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

        pub fn juggle(mut hands: Hands) -> Hands {
            print!("Juggling ...");
            let air = hands.left;
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
                println!("\n{} hand is holding {} ", which, item.what);
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

     // create a new student into the student struct

     let name = String::from("John");
        let age = 20;
        let marks = 80;
        let grade = 'A';
        
    let mut student = model::StudentDetails::new(String::from(name),  age, marks, grade);
    println!(
        "\nStudent name is {} and age is {} and marks is {} ",
        student.name, student.age, student.marks
    );

    //set student details
    student.set_student_details(String::from("Kariuki"), 26, 90, 'A');

    //print the mut student details
    println!(
        "\nStudent name is {} and age is {} and marks is {} ",
        student.name, student.age, student.marks
    );

    //get all student details
    println!(" \n{}", student.get_student_details());


    //add 3 students into the student struct
    let student1 = model::StudentDetails::add_student(String::from("John"), 20, 80, 'A');
    let student2 = model::StudentDetails::add_student(String::from("Kariuki"), 26, 90, 'A');
    let student3 = model::StudentDetails::add_student(String::from("Kamau"), 30, 70, 'B');

    //print all the student details
    println!(" \n{}", student1.get_student_details());
    println!(" \n{}", student2.get_student_details());
    println!(" \n{}", student3.get_student_details());


    //delete student details
    student.delete_student();

    //print the mut student details
    println!(
        "\nStudent name is {} and age is {} and marks is {} ",
        student.name, student.age, student.marks
    );

    //delete a specific student details
    student.delete_specific_student(String::from("Kariuki"));
    
   
    
}
