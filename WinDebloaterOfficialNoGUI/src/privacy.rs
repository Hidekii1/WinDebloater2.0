// Módulo para protección de privacidad
use crate::utils::{
    disable_scheduled_task, disable_service, print_error, print_header, print_info, print_success,
    run_powershell,
};

/// Servicios de telemetría a desactivar
const TELEMETRY_SERVICES: &[&str] = &[
    "DiagTrack",        // Servicio de seguimiento de diagnóstico
    "dmwappushservice", // Servicio de envío de datos WAP
    "WMPNetworkSvc",    // Servicio de red de Windows Media Player
    "SysMain",          // Superfetch (puede consumir recursos)
    "WSearch",          // Windows Search (opcional, consume recursos)
];

/// Tareas programadas de telemetría
const TELEMETRY_TASKS: &[&str] = &[
    "\\Microsoft\\Windows\\Customer Experience Improvement Program\\Consolidator",
    "\\Microsoft\\Windows\\Customer Experience Improvement Program\\UsbCeip",
    "\\Microsoft\\Windows\\Application Experience\\ProgramDataUpdater",
    "\\Microsoft\\Windows\\Application Experience\\Microsoft Compatibility Appraiser",
    "\\Microsoft\\Windows\\Autochk\\Proxy",
    "\\Microsoft\\Windows\\DiskDiagnostic\\Microsoft-Windows-DiskDiagnosticDataCollector",
];

pub fn run_privacy_protection() {
    print_header("Protección de Privacidad");

    // Desactivar servicios de telemetría
    print_info("Desactivando servicios de telemetría...");
    for service in TELEMETRY_SERVICES {
        disable_service(service);
    }

    println!();

    // Desactivar tareas programadas
    print_info("Desactivando tareas programadas de telemetría...");
    for task in TELEMETRY_TASKS {
        disable_scheduled_task(task);
    }

    println!();

    // Desactivar telemetría en el registro
    print_info("Configurando registro para máxima privacidad...");

    let registry_commands = [
        // Desactivar telemetría
        (
            "HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection",
            "AllowTelemetry",
            "0",
        ),
        // Desactivar Advertising ID
        (
            "HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo",
            "Enabled",
            "0",
        ),
        // Desactivar actividad del timeline
        (
            "HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\System",
            "EnableActivityFeed",
            "0",
        ),
        // Desactivar feedback
        (
            "HKCU:\\SOFTWARE\\Microsoft\\Siuf\\Rules",
            "NumberOfSIUFInPeriod",
            "0",
        ),
    ];

    for (path, name, value) in registry_commands {
        let cmd = format!(
            "New-Item -Path '{}' -Force -ErrorAction SilentlyContinue | Out-Null; Set-ItemProperty -Path '{}' -Name '{}' -Value {} -Force",
            path, path, name, value
        );
        match run_powershell(&cmd) {
            Ok(_) => print_success(&format!("{} configurado", name)),
            Err(e) if e.is_empty() => print_success(&format!("{} configurado", name)),
            Err(e) => print_error(&format!("Error en {}: {}", name, e.trim())),
        }
    }

    println!("\n  Protección de privacidad aplicada correctamente.");
}
