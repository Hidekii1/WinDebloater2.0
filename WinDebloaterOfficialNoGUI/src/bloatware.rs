// Módulo para eliminación de bloatware
use crate::utils::{print_header, remove_app};

/// Lista de aplicaciones bloatware comunes en Windows
const BLOATWARE_APPS: &[(&str, &str)] = &[
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
    ("Microsoft.WindowsAlarms", "Alarmas"),
    ("Microsoft.WindowsFeedbackHub", "Centro de comentarios"),
    ("Microsoft.WindowsMaps", "Mapas"),
    ("Microsoft.WindowsSoundRecorder", "Grabadora de sonido"),
    ("Microsoft.YourPhone", "Tu Teléfono"),
    ("Microsoft.MSPaint", "Paint 3D"),
    ("Microsoft.BingWeather", "El Tiempo"),
    ("Clipchamp.Clipchamp", "Clipchamp"),
    ("Microsoft.Todos", "Microsoft To Do"),
    ("Microsoft.PowerAutomateDesktop", "Power Automate"),
    ("MicrosoftTeams", "Microsoft Teams"),
    ("Microsoft.549981C3F5F10", "Cortana"),
];

pub fn run_bloatware_removal() {
    print_header("Eliminación de Bloatware");
    println!(
        "  Eliminando {} aplicaciones preinstaladas...\n",
        BLOATWARE_APPS.len()
    );

    for (package, name) in BLOATWARE_APPS {
        remove_app(package, name);
    }

    println!("\n  Proceso de eliminación de bloatware completado.");
}
