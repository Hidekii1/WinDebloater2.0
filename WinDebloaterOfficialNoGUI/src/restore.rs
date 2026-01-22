// Módulo para restaurar cambios
use crate::utils::{
    enable_scheduled_task, enable_service, print_error, print_header, print_info, print_success,
    run_powershell,
};
use std::process::Command;

/// Servicios a restaurar
const SERVICES_TO_RESTORE: &[&str] = &["DiagTrack", "dmwappushservice", "WMPNetworkSvc", "SysMain"];

/// Tareas programadas a restaurar
const TASKS_TO_RESTORE: &[&str] = &[
    "\\Microsoft\\Windows\\Customer Experience Improvement Program\\Consolidator",
    "\\Microsoft\\Windows\\Customer Experience Improvement Program\\UsbCeip",
    "\\Microsoft\\Windows\\Application Experience\\ProgramDataUpdater",
    "\\Microsoft\\Windows\\Application Experience\\Microsoft Compatibility Appraiser",
];

pub fn run_restore() {
    print_header("Restauración");
    print_info("Revirtiendo cambios realizados por WinDebloater...\n");

    // Restaurar servicios
    print_info("Restaurando servicios...");
    for service in SERVICES_TO_RESTORE {
        enable_service(service);
    }

    println!();

    // Restaurar tareas programadas
    print_info("Restaurando tareas programadas...");
    for task in TASKS_TO_RESTORE {
        enable_scheduled_task(task);
    }

    println!();

    // Restaurar telemetría en el registro
    print_info("Restaurando configuración de telemetría...");
    let telemetry_cmd = "Set-ItemProperty -Path 'HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection' -Name 'AllowTelemetry' -Value 3 -Force";
    match run_powershell(telemetry_cmd) {
        Ok(_) => print_success("Telemetría restaurada"),
        Err(e) if e.is_empty() => print_success("Telemetría restaurada"),
        Err(e) => print_error(&format!("Error restaurando telemetría: {}", e.trim())),
    }

    // Restaurar animaciones
    print_info("Restaurando efectos visuales...");
    let anim_cmd = r#"
        Set-ItemProperty -Path 'HKCU:\Control Panel\Desktop' -Name 'UserPreferencesMask' -Value ([byte[]](0x9e,0x3e,0x07,0x80,0x12,0x00,0x00,0x00)) -Force
        Set-ItemProperty -Path 'HKCU:\Control Panel\Desktop\WindowMetrics' -Name 'MinAnimate' -Value '1' -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'TaskbarAnimations' -Value 1 -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects' -Name 'VisualFXSetting' -Value 0 -Force
    "#;
    match run_powershell(anim_cmd) {
        Ok(_) => print_success("Efectos visuales restaurados"),
        Err(e) if e.is_empty() => print_success("Efectos visuales restaurados"),
        Err(e) => print_error(&format!("Error en efectos visuales: {}", e.trim())),
    }

    // Restaurar inicio rápido
    match run_powershell("powercfg /hibernate on") {
        Ok(_) => print_success("Inicio rápido restaurado"),
        Err(_) => {}
    }

    // Restaurar menú contextual de Windows 11
    print_info("Restaurando menú contextual de Windows 11...");
    let result = Command::new("reg")
        .args([
            "delete",
            "HKCU\\Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}",
            "/f",
        ])
        .output();
    match result {
        Ok(o) if o.status.success() => print_success("Menú contextual de Windows 11 restaurado"),
        _ => print_info("Menú contextual ya estaba en configuración por defecto"),
    }

    // Restaurar configuración de barra de tareas
    print_info("Restaurando barra de tareas...");
    let taskbar_restore = r#"
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'TaskbarDa' -Value 1 -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'TaskbarMn' -Value 1 -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'ShowTaskViewButton' -Value 1 -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'TaskbarAl' -Value 1 -Force
    "#;
    match run_powershell(taskbar_restore) {
        Ok(_) => print_success("Barra de tareas restaurada"),
        Err(e) if e.is_empty() => print_success("Barra de tareas restaurada"),
        Err(e) => print_error(&format!("Error en barra de tareas: {}", e.trim())),
    }

    println!("\n  Restauración completada.");
    print_info("Reinicia el sistema para aplicar todos los cambios.");
}
