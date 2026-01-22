// Módulo para personalización (selección de apps a eliminar)
use crate::utils::{print_header, print_info, print_warning, remove_app};
use std::io::{self, Write};

/// Lista de aplicaciones disponibles para eliminar
const AVAILABLE_APPS: &[(&str, &str)] = &[
    ("Microsoft.ZuneMusic", "Groove Música"),
    ("Microsoft.ZuneVideo", "Películas y TV"),
    ("Microsoft.XboxApp", "Xbox"),
    ("Microsoft.XboxGameOverlay", "Xbox Game Overlay"),
    ("Microsoft.XboxGamingOverlay", "Xbox Gaming Overlay"),
    ("Microsoft.XboxIdentityProvider", "Xbox Identity Provider"),
    ("Microsoft.XboxSpeechToTextOverlay", "Xbox Speech to Text"),
    ("Microsoft.GetHelp", "Obtener ayuda"),
    ("Microsoft.Getstarted", "Primeros pasos"),
    ("Microsoft.Microsoft3DViewer", "3D Viewer"),
    ("Microsoft.MicrosoftOfficeHub", "Office Hub"),
    ("Microsoft.MicrosoftSolitaireCollection", "Solitario"),
    ("Microsoft.MicrosoftStickyNotes", "Notas rápidas"),
    ("Microsoft.OneConnect", "OneConnect"),
    ("Microsoft.People", "Contactos"),
    ("Microsoft.SkypeApp", "Skype"),
    ("Microsoft.Wallet", "Wallet"),
    ("Microsoft.WindowsAlarms", "Alarmas y reloj"),
    ("Microsoft.WindowsFeedbackHub", "Centro de comentarios"),
    ("Microsoft.WindowsMaps", "Mapas"),
    ("Microsoft.WindowsSoundRecorder", "Grabadora de sonido"),
    ("Microsoft.YourPhone", "Tu Teléfono / Phone Link"),
    ("Microsoft.MSPaint", "Paint 3D"),
    ("Microsoft.BingWeather", "El Tiempo"),
    ("Clipchamp.Clipchamp", "Clipchamp"),
    ("Microsoft.Todos", "Microsoft To Do"),
    ("MicrosoftTeams", "Microsoft Teams"),
    ("Microsoft.549981C3F5F10", "Cortana"),
    ("Microsoft.WindowsCamera", "Cámara"),
    ("Microsoft.WindowsCalculator", "Calculadora"),
];

pub fn run_customization() {
    print_header("Personalización");
    println!("  Selecciona las aplicaciones que deseas eliminar:\n");

    // Mostrar lista de aplicaciones
    for (i, (_, name)) in AVAILABLE_APPS.iter().enumerate() {
        println!("  {:2}. {}", i + 1, name);
    }

    println!("\n   0. Cancelar");
    println!("   A. Eliminar todas las aplicaciones listadas\n");

    print!("  Ingresa los números separados por coma (ej: 1,3,5) o 'A' para todas: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        print_warning("Error leyendo entrada");
        return;
    }

    let input = input.trim();

    // Manejar cancelación
    if input == "0" {
        print_info("Operación cancelada");
        return;
    }

    // Manejar "eliminar todas"
    if input.eq_ignore_ascii_case("a") {
        println!();
        print_warning("Eliminando TODAS las aplicaciones listadas...\n");
        for (package, name) in AVAILABLE_APPS {
            remove_app(package, name);
        }
        println!("\n  Todas las aplicaciones han sido procesadas.");
        return;
    }

    // Procesar selección individual
    let indices: Vec<usize> = input
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .filter(|&i| i > 0 && i <= AVAILABLE_APPS.len())
        .collect();

    if indices.is_empty() {
        print_warning("No se seleccionaron aplicaciones válidas");
        return;
    }

    println!();
    for i in indices {
        let (package, name) = AVAILABLE_APPS[i - 1];
        remove_app(package, name);
    }

    println!("\n  Personalización completada.");
}
