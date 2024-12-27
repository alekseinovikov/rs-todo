use crate::gui::Draw;
use crate::task::ShortTask;
use imgui::Ui;

pub(crate) struct TaskList {
    undone_tasks: Vec<ShortTask>,
}

impl TaskList {
    pub(crate) fn new(undone_tasks: Vec<ShortTask>) -> TaskList {
        TaskList { undone_tasks }
    }
}

impl Draw for ShortTask {
    fn draw(&self, ui: &Ui) {
        ui.text(&self.title);
    }
}

impl Draw for TaskList {
    fn draw(&self, ui: &Ui) {
        for task in &self.undone_tasks {
            task.draw(ui);
        }
    }
}
