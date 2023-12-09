// Importando biblioteca fmt
use std::fmt;

// Definiendo la estructura TodoList
pub struct TodoList {
    tasks: Vec<Task>,

}

// Definiendo la estructura Task
pub struct Task {
    description: String,
    due_time: Option<String>,
    completed: bool,
}
// Implementación de metódos
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
            if self.completed { "✅" } else { "[ ]" },
            self.description
        )?;
        if let Some(due_time) = &self.due_time {
            write!(f, " - Hora Limite: {}", due_time)?;
        }
        Ok(())
    }
}
// Implementación de métodos
impl TodoList {
    // Crear nueva lista de tareas
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }
    // Metódo que agrega una nueva tarea a la lista
    pub fn new_task(&mut self, description: String, due_time: Option<String>) {
        let task = Task::new(description, due_time);
        self.tasks.push(task);
    }
    // Metodo que completa tareas de la lista
    pub fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.complete();
            println!("Tarea marcada como completada: {}", task);
        } else {
            println!("Índice de tarea no válido.");
        }
    }
    // Metodo que elimina una tarea de la lista
    pub fn delete_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            let removed_task = self.tasks.remove(index);
            println!("Tarea eliminada: {}", removed_task);
        } else {
            println!("Índice de tarea no válido.");
        }
    }
    // Metódo que muestra las tareas pendientes
    pub fn show_task(&self) {
        if self.tasks.is_empty() {
            println!("No hay tareas pendientes.");
        } else {
            println!("Tareas pendientes:");
            for (index, task) in self.tasks.iter().enumerate() {
                println!("{}. {}", index + 1, task);
            }
        }
    }
    // Métodos para obtener el número de tareas completadas y el total de tareas
    pub fn completed_tasks(&self) -> usize {
        self.tasks.iter().filter(|task| task.completed).count()
    }

    pub fn total_tasks(&self) -> usize {
        self.tasks.len()
    }
    // Método para verificar si todas las tareas están completadas
    pub fn is_completed(&self) -> bool {
        self.tasks.iter().all(|task| task.completed)
    }
}


