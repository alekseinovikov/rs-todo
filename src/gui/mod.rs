mod support;

use crate::service::Service;
use crate::task::ShortTask;
use imgui::Condition;

struct ViewState {
    undone_tasks: Vec<ShortTask>,
}

pub struct Gui {
    service: Box<dyn Service>,
    state: ViewState,
}

impl Gui {
    pub fn new(service: Box<dyn Service>) -> Gui {
        let tasks = service.get_all_undone().unwrap();
        Gui {
            service,
            state: ViewState { undone_tasks: tasks },
        }
    }

    pub fn run(&self) {
        let mut value = 0;
        let choices = ["test test this is 1", "test test this is 2"];
        support::simple_init("TODO", move |_, ui| {
            ui.window("Tasks")
                .collapsible(false)
                .resizable(false)
                .position([0.0, 0.0], Condition::Always)
                .size([1024.0, 768.0], Condition::Always)
                .title_bar(false)
                .build(|| {
                    ui.same_line();
                    ui.checkbox("Show Done", &mut false);
                    ui.same_line();
                    ui.label_text("Filter", "Filter by:");
                    ui.separator();
                });
        });
    }
}
