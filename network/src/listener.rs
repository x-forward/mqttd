use std::error::Error;
pub trait Listener {
    fn close(&self);

    fn serve(&self);

    fn name(&self);
}

pub struct Listeners {}

impl Listeners {
    pub fn new() -> Listeners {
        todo!()
    }

    pub fn add(&mut self, name: String, l: Box<dyn Listener>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
