mod list;
mod state;
mod support;
mod details;

use crate::gui::state::ViewState;
use crate::service::Service;

pub struct Gui {
    state: ViewState,
}

impl Gui {
    pub fn new(service: Box<dyn Service>) -> Gui {
        Gui {
            state: ViewState::new(service),
        }
    }

    pub fn run(self) {
        let mut state = self.state;
        support::simple_init("TODO", move |_, ui| {
            state.draw(ui);
        });
    }
}
