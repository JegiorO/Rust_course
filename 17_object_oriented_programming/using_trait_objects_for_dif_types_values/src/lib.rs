// lib example of structure of program with gui 
// make ui component drawable 
pub trait Draw {
    fn draw(&self);
}

pub struct Screen { //making componentes of drawer drawable
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        //drawing each component
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

//making button drawable with impl
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}