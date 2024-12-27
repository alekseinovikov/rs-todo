use crate::service::Service;

pub struct Gui {
    service: Box<dyn Service>,
}

impl Gui {
    pub fn new(service: Box<dyn Service>) -> Gui {
        Gui { service }
    }

    pub fn run(&self) {
        println!("Running GUI with service");
    }
}
