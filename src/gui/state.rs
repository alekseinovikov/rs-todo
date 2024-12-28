use crate::gui::details::DetailedView;
use crate::gui::list::TaskList;
use crate::service::Service;
use imgui::{Condition, Ui};
use std::ops::Not;

pub(crate) struct ViewState {
    service: Box<dyn Service>,
    list: TaskList,
    detailed_view: DetailedView,
}

impl ViewState {
    pub(crate) fn new(service: Box<dyn Service>) -> ViewState {
        let tasks = service.get_all().unwrap();
        ViewState {
            service,
            list: TaskList::new(tasks),
            detailed_view: DetailedView::new(None),
        }
    }
}

impl ViewState {
    pub fn draw(&mut self, ui: &mut Ui) {
        ui.window("Tasks")
            .collapsible(false)
            .resizable(false)
            .position([0.0, 0.0], Condition::Always)
            .size([512.0, 768.0], Condition::Always)
            .build(|| {
                self.list.draw(ui);
            });

        let selected_task_id = self.list.get_selected_task_id();
        if (self.detailed_view.is_the_same(selected_task_id).not() && selected_task_id.is_some()) {
            let selected_task = self.service.get(selected_task_id.unwrap()).unwrap();
            match selected_task {
                Some(selected_task) => {
                    self.detailed_view.set_task(&selected_task);
                }
                None => {}
            }
        }

        ui.window("Detailed View")
            .collapsible(false)
            .resizable(false)
            .position([512.0, 0.0], Condition::Always)
            .size([512.0, 768.0], Condition::Always)
            .build(|| {
                self.detailed_view.draw(ui);
            });
    }
}
