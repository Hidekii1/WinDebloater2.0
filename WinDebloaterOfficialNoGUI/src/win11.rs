// Módulo para ajustes específicos de Windows 11
use crate::utils::{print_error, print_header, print_info, print_success, run_powershell};
use std::process::Command;

pub fn run_win11_tweaks() {
    print_header("Ajustes para Windows 11");

    // Restaurar menú contextual clásico
    print_info("Restaurando menú contextual clásico...");
    let result = Command::new("reg")
        .args([
            "add",
            "HKCU\\Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32",
            "/f",
            "/ve",
        ])
        .output();
    match result {
        Ok(o) if o.status.success() => print_success("Menú contextual clásico restaurado"),
        _ => print_error("No se pudo restaurar el menú contextual"),
    }

    // Desactivar widgets
    print_info("Configurando barra de tareas...");
    let taskbar_tweaks = r#"
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'TaskbarDa' -Value 0 -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'TaskbarMn' -Value 0 -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'ShowTaskViewButton' -Value 0 -Force
    "#;
    match run_powershell(taskbar_tweaks) {
        Ok(_) => {
            print_success("Widgets desactivados");
            print_success("Chat desactivado");
            print_success("Vista de tareas desactivada");
        }
        Err(e) if e.is_empty() => {
            print_success("Widgets desactivados");
            print_success("Chat desactivado");
            print_success("Vista de tareas desactivada");
        }
        Err(e) => print_error(&format!("Error en barra de tareas: {}", e.trim())),
    }

    // Alinear barra de tareas a la izquierda
    print_info("Ajustando alineación de la barra de tareas...");
    match run_powershell(
        "Set-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced' -Name 'TaskbarAl' -Value 0 -Force",
    ) {
        Ok(_) => print_success("Barra de tareas alineada a la izquierda"),
        Err(e) if e.is_empty() => print_success("Barra de tareas alineada a la izquierda"),
        Err(e) => print_error(&format!("Error en alineación: {}", e.trim())),
    }

    // Desactivar búsqueda destacada
    print_info("Configurando búsqueda...");
    match run_powershell(
        "Set-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\SearchSettings' -Name 'IsDynamicSearchBoxEnabled' -Value 0 -Force",
    ) {
        Ok(_) => print_success("Búsqueda destacada desactivada"),
        Err(e) if e.is_empty() => print_success("Búsqueda destacada desactivada"),
        Err(_) => {}
    }

    // Mostrar extensiones de archivo
    print_info("Configurando explorador de archivos...");
    let explorer_tweaks = r#"
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'HideFileExt' -Value 0 -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'Hidden' -Value 1 -Force
    "#;
    match run_powershell(explorer_tweaks) {
        Ok(_) => {
            print_success("Extensiones de archivo visibles");
            print_success("Archivos ocultos visibles");
        }
        Err(e) if e.is_empty() => {
            print_success("Extensiones de archivo visibles");
            print_success("Archivos ocultos visibles");
        }
        Err(e) => print_error(&format!("Error en explorador: {}", e.trim())),
    }

    println!("\n  Ajustes de Windows 11 aplicados.");
    print_info("Reinicia el explorador o el sistema para ver todos los cambios.");
}
