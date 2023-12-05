use std::io::{self, Write};
use To_Do_List::ToDo;

fn main() {
    let mut todo_list = ToDo::new();

    loop {
        print_menu();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim() {
            "1" => {
                println!("Ingrese la tarea:");
                let mut task_input = String::new();
                io::stdin().read_line(&mut task_input).expect("Error al leer la tarea");
                todo_list.new_task(task_input.trim().to_string());
            }
            "2" => todo_list.show_tasks(),
            "q" => {
                println!("Saliendo del programa.");
                break;
            }
            _ => println!("Opción no válida. Inténtelo de nuevo."),
        }
    }
}

fn print_menu() {
    println!("------ Menú ------");
    println!("1. Agrega tarea");
    println!("2. Mostrar tareas pendientes");
    println!("q. Salir");
    print!("Seleccione una opción: ");
    io::stdout().flush().unwrap();
}
