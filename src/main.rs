// Importando bibliotecas 
use std::io::{self, Write};
use To_Do_List::{TodoList, Task};

// Función principal
fn main() {
    // Creando nueva lista de tareas
    let mut todo_list = TodoList::new();

    // Bucle principal
    loop {
        // Mostrando el menú de opciones
        print_menu();

        // Leyendo la entrada del usuario
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        // Manejo de opciones ingresadas
        match input.trim() {
            "1" => {
                // Agregar nueva tarea
                // Se solicita la hora límite "opcional"
                println!("¡Nueva tarea!");
                let mut task_input = String::new();
                io::stdin().read_line(&mut task_input).expect("Error al leer la tarea");

                println!("¿Tienes una hora límite para completar la tarea? (S/N)");
                let mut time_limit = String::new();
                io::stdin()
                    .read_line(&mut time_limit)
                    .expect("Error al leer la entrada");

                let due_time = if time_limit.trim().to_lowercase() == "s" {
                    println!("Ingresa la hora límite (por ejemplo, '10:30'): ");
                    let mut time_limit_input = String::new();
                    io::stdin()
                        .read_line(&mut time_limit_input)
                        .expect("Error al leer la hora límite");
                    Some(time_limit_input.trim().to_string())
                } else {
                    None
                };

                todo_list.new_task(task_input.trim().to_string(), due_time);
            }
            "2" => {
                // Mostrar tareas pendientes
                todo_list.show_task();
            }
            "3" => {
                // Marcar tareas como completas
                println!("Marcar tarea como completada:");
                let mut index_input = String::new();
                io::stdin()
                    .read_line(&mut index_input)
                    .expect("Error al leer el índice de la tarea");

                if let Ok(index) = index_input.trim().parse::<usize>() {
                    todo_list.complete_task(index - 1);
                    print_progress(&todo_list);
                    check_completion(&todo_list);
                } else {
                    println!("Índice no válido. Inténtalo de nuevo.");
                }
            }
            "4" => {
                // Eliminar tarea
                println!("Eliminar tarea:");
                let mut index_input = String::new();
                io::stdin()
                    .read_line(&mut index_input)
                    .expect("Error al leer el índice de la tarea");

                if let Ok(index) = index_input.trim().parse::<usize>() {
                    todo_list.delete_task(index - 1);
                    print_progress(&todo_list);
                    check_completion(&todo_list);
                } else {
                    println!("Índice no válido. Inténtalo de nuevo.");
                }
            }
            "5" => {
                // Mostrar recompensas
                print_rewards();
            }
            "x" => {
                // Salir del programa
                println!("Saliendo del programa.");
                break;
            }
            _ => {
                // Opción no válida
                println!("Opción no válida. Inténtalo de nuevo.");
            }
        }
    }

    fn check_completion(todo_list: &TodoList) {
        if todo_list.is_completed() {
            println!("¡Felicidades! Has completado todas las tareas.");
            print_rewards();
        }
    }
}

// Función para imprimir el menú
fn print_menu() {
    println!("------ Menú ------");
    println!("1. ¡Agregar Nueva Tarea!");
    println!("2. Mostrar Tareas Pendientes");
    println!("3. Marcar Tarea como Completada");
    println!("4. Eliminar Tarea");
    println!("5. Recompensas");
    println!("x. Salir");
    print!("Selecciona una opción: ");
    io::stdout().flush().unwrap();
}

// Función para imprimir el progreso de las tareas completadas
fn print_progress(todo_list: &TodoList) {
    let total_tasks = todo_list.total_tasks();
    let completed_tasks = todo_list.completed_tasks();
    let progress = (completed_tasks as f32 / total_tasks as f32) * 100.0;
    println!("Progreso: {:.2}%", progress);
}

// Función para imprimir recompensas
fn print_rewards() {
    println!("------ Recompensas ------");
    println!("🌟 ¡Has completado una tarea! ¡Bien hecho! 🌟");
    println!("🏆 ¡Sigue así y alcanzarás todas tus metas! 🏆");
    println!("-------------------------");
}


