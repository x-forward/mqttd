use crate::listener::Listener;
pub struct Tcp {}

impl Listener for Tcp {
    fn close(&self) {
        todo!()
    }

    fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}
