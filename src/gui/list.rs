use crate::task::ShortTask;
use imgui::Ui;

pub(crate) struct TaskList {
    tasks: Vec<ShortTask>,
    selected_index: Option<i32>,
    selected_item: Option<ShortTask>,
}

impl TaskList {
    pub(crate) fn new(undone_tasks: Vec<ShortTask>) -> TaskList {
        TaskList {
            tasks: undone_tasks,
            selected_index: None,
            selected_item: None,
        }
    }
}

impl TaskList {

    pub(crate) fn get_selected_task_id(&self) -> Option<i32> {
        self.selected_item.as_ref().map(|task| task.id)
    }

    pub fn draw(&mut self, ui: &Ui) {
        let mut selected_item_index = self.selected_index.unwrap_or(-1);
        let items: Vec<&str> = self.tasks.iter().map(|task| task.title.as_str()).collect();

        if (ui.list_box(
            "Tasks",
            &mut selected_item_index,
            &items,
            items.len() as i32,
        )) {
            self.selected_index = Some(selected_item_index);
            self.selected_item = self
                .tasks
                .get(selected_item_index as usize)
                .map(|task| task.clone());
        }
    }
}
