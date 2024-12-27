mod list;
mod state;
mod support;

use crate::gui::state::ViewState;
use crate::service::Service;
use imgui::Ui;

pub struct Gui {
    state: ViewState,
}

trait Draw {
    fn draw(&self, ui: &Ui);
}

trait WindowDraw {
    fn draw(&self, ui: &mut Ui);
}

impl Gui {
    pub fn new(service: Box<dyn Service>) -> Gui {
        Gui {
            state: ViewState::new(service),
        }
    }

    pub fn run(self) {
        let state = self.state;
        support::simple_init("TODO", move |_, ui| {
            state.draw(ui);
        });
    }
}
