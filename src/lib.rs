use std::fmt;

pub struct TodoList {
    tasks: Vec<Task>,
}

pub struct Task {
    description: String,
    due_time: Option<String>,
    completed: bool,
}

impl Task {
    pub fn new(description: String, due_time: Option<String>) -> Self {
        Task {
            description,
            due_time,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}",
            if self.completed { "[✔]" } else { "[ ]" },
            self.description
        )?;
        if let Some(due_time) = &self.due_time {
            write!(f, " - Hora Limite: {}", due_time)?;
        }
        Ok(())
    }
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    pub fn new_task(&mut self, description: String, due_time: Option<String>) {
        let task = Task::new(description, due_time);
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.complete();
            println!("Tarea marcada como completada: {}", task);
        } else {
            println!("Índice de tarea no válido.");
        }
    }

    pub fn delete_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            let removed_task = self.tasks.remove(index);
            println!("Tarea eliminada: {}", removed_task);
        } else {
            println!("Índice de tarea no válido.");
        }
    }

    pub fn display_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No hay tareas pendientes.");
        } else {
            println!("Tareas pendientes:");
            for (index, task) in self.tasks.iter().enumerate() {
                println!("{}. {}", index + 1, task);
            }
        }
    }
}
