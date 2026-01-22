mod bloatware;
mod customize;
mod optimize;
mod privacy;
mod restore;
mod utils;
mod win11;

use colored::Colorize;
use std::io::{self, Write};
use utils::{is_admin, print_banner, print_warning};

fn main() {
    print_banner();

    // Verificar permisos de administrador
    if !is_admin() {
        print_warning("ADVERTENCIA: No se está ejecutando como Administrador.");
        print_warning("Algunas funciones pueden no funcionar correctamente.\n");
    }

    loop {
        println!("\n{}", "Menú Principal".cyan().bold());
        println!("─────────────────────────────────────");
        println!("  {}. Eliminación de Bloatware", "1".green());
        println!("  {}. Protección de Privacidad", "2".green());
        println!("  {}. Optimización de Windows", "3".green());
        println!("  {}. Ajustes para Windows 11", "4".green());
        println!("  {}. Personalización", "5".green());
        println!("  {}. Restauración", "6".green());
        println!("─────────────────────────────────────");
        println!("  {}. Salir", "0".red());
        println!();

        print!("  Selecciona una opción: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        if io::stdin().read_line(&mut option).is_err() {
            println!("Error leyendo entrada. Intenta de nuevo.");
            continue;
        }

        match option.trim() {
            "1" => bloatware::run_bloatware_removal(),
            "2" => privacy::run_privacy_protection(),
            "3" => optimize::run_optimization(),
            "4" => win11::run_win11_tweaks(),
            "5" => customize::run_customization(),
            "6" => restore::run_restore(),
            "0" => {
                println!("\n  {} ¡Hasta luego!\n", "👋".to_string());
                break;
            }
            _ => println!("\n  {} Opción no válida. Intenta de nuevo.", "⚠".yellow()),
        }

        // Pausa antes de mostrar el menú de nuevo
        println!("\n  Presiona Enter para continuar...");
        let _ = io::stdin().read_line(&mut String::new());
    }
}
