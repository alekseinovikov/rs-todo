use crate::task::Task;
use imgui::Ui;

pub(crate) struct DetailedView {
    task: Option<Task>,
}

impl DetailedView {
    pub(crate) fn new(task: Option<Task>) -> DetailedView {
        DetailedView { task }
    }

    pub(crate) fn is_the_same(&self, task_id: Option<i32>) -> bool {
        if (self.task.is_none() && task_id.is_none()) {
            return true;
        }

        if (self.task.is_none() || task_id.is_none()) {
            return false;
        }

        let task_id = task_id.unwrap();
        let self_task = self.task.as_ref().unwrap();
        self_task.id == task_id
    }

    pub(crate) fn set_task(&mut self, task: &Task) {
        self.task = Some(task.clone());
    }

    pub(crate) fn draw(&self, ui: &Ui) {
        if (self.task.is_none()) {
            return;
        }

        let task = self.task.as_ref().unwrap();
        ui.same_line();
        ui.text("Title:");
        ui.same_line();
        ui.text(task.title.as_str());

        ui.text("Description:");
        ui.same_line();
        ui.text(task.description.as_str());

        let mut done = task.done;
        ui.checkbox("Done", &mut done);
    }
}
