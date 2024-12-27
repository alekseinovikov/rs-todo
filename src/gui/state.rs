use crate::gui::list::TaskList;
use crate::gui::{Draw, WindowDraw};
use crate::service::Service;
use imgui::{Condition, Ui};

pub(crate) struct ViewState {
    service: Box<dyn Service>,
    list: TaskList,
}

impl ViewState {
    pub(crate) fn new(service: Box<dyn Service>) -> ViewState {
        let tasks = service.get_all_undone().unwrap();
        ViewState {
            service: service,
            list: TaskList::new(vec![]),
        }
    }

    pub(crate) fn load_undone_tasks(&mut self) {
        self.list = TaskList::new(self.service.get_all_undone().unwrap());
    }
}

impl WindowDraw for ViewState {
    fn draw(&self, ui: &mut Ui) {
        ui.window("Tasks")
            .collapsible(false)
            .resizable(false)
            .position([0.0, 0.0], Condition::Always)
            .size([512.0, 768.0], Condition::Always)
            .build(|| {
                self.list.draw(ui);
            });

        ui.window("Description")
            .collapsible(false)
            .resizable(false)
            .position([512.0, 0.0], Condition::Always)
            .size([512.0, 768.0], Condition::Always)
            .build(|| {
                self.list.draw(ui);
            });
    }
}
