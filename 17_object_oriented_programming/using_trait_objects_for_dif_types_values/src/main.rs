use using_trait_objects_for_dif_types_values::Draw;

//if someone decides to add new type, he can easily impement it!
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

//via using our trait
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use using_trait_objects_for_dif_types_values::{Button, Screen};

//initializing drawer and using it
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}