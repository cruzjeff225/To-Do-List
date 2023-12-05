pub struct ToDo {
    tasks: Vec<String>,
}

impl ToDo {
    pub fn new()-> ToDo {
        ToDo { tasks: Vec::new() }
    }

    pub fn new_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn show_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No hay tareas pendientes.");
        } else {
            println!("Tareas pendientes:");
            self.tasks.iter().enumerate().for_each(|(index, task)| {
                println!("{}. {}", index + 1, task);
            });
        }
    }
}
