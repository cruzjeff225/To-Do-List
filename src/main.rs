use std::io::{self, Write};
use To_Do_List::{TodoList, Task};

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        print_menu();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim() {
            "1" => {
                println!("Ingrese la tarea:");
                let mut task_input = String::new();
                io::stdin().read_line(&mut task_input).expect("Error al leer la tarea");

                println!("¿Tiene una hora límite para completar la tarea? (S/N)");
                let mut has_due_time_input = String::new();
                io::stdin()
                    .read_line(&mut has_due_time_input)
                    .expect("Error al leer la entrada");

                let due_time = if has_due_time_input.trim().to_lowercase() == "s" {
                    println!("Ingrese la hora límite (por ejemplo, '14:30'): ");
                    let mut due_time_input = String::new();
                    io::stdin()
                        .read_line(&mut due_time_input)
                        .expect("Error al leer la hora límite");
                    Some(due_time_input.trim().to_string())
                } else {
                    None
                };

                todo_list.new_task(task_input.trim().to_string(), due_time);
            }
            "2" => {
                todo_list.display_tasks();
            }
            "3" => {
                println!("Ingrese el número de la tarea que desea marcar como completada:");
                let mut index_input = String::new();
                io::stdin()
                    .read_line(&mut index_input)
                    .expect("Error al leer el índice de la tarea");

                if let Ok(index) = index_input.trim().parse::<usize>() {
                    todo_list.complete_task(index - 1);
                } else {
                    println!("Índice no válido. Inténtelo de nuevo.");
                }
            }
            "q" => {
                println!("Saliendo del programa.");
                break;
            }
            _ => {
                println!("Opción no válida. Inténtelo de nuevo.");
            }
        }
    }
}

fn print_menu() {
    println!("------ Menú ------");
    println!("1. Agregar tarea");
    println!("2. Mostrar tareas");
    println!("3. Marcar tarea como completada");
    println!("q. Salir");
    print!("Seleccione una opción: ");
    io::stdout().flush().unwrap();
}