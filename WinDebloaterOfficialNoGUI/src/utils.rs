// Módulo de utilidades compartidas
use colored::Colorize;
use std::process::Command;

/// Ejecuta un comando PowerShell y retorna el resultado
pub fn run_powershell(command: &str) -> Result<String, String> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", command])
        .output();

    match output {
        Ok(o) if o.status.success() => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
        Ok(o) => Err(String::from_utf8_lossy(&o.stderr).to_string()),
        Err(e) => Err(e.to_string()),
    }
}

/// Verifica si una app está instalada
fn is_app_installed(package_name: &str) -> bool {
    let cmd = format!(
        "Get-AppxPackage -AllUsers | Where-Object {{ $_.Name -like '*{}*' }} | Select-Object -First 1",
        package_name
    );
    match run_powershell(&cmd) {
        Ok(output) => !output.trim().is_empty(),
        Err(_) => false,
    }
}

/// Elimina una app de Windows usando múltiples métodos agresivos
/// Verifica después de cada método si la app fue eliminada
pub fn remove_app(package_name: &str, display_name: &str) {
    // Primero verificar si la app está instalada
    if !is_app_installed(package_name) {
        print_success(&format!("'{}' no está instalada", display_name));
        return;
    }

    print_info(&format!("Eliminando '{}'...", display_name));

    // Método 1: Remover paquete instalado para todos los usuarios
    let cmd1 = format!(
        "Get-AppxPackage -AllUsers '*{}*' | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue 2>$null",
        package_name
    );
    let _ = run_powershell(&cmd1);
    std::thread::sleep(std::time::Duration::from_millis(300));
    if !is_app_installed(package_name) {
        print_success(&format!(
            "'{}' eliminada [Método: Remove-AppxPackage]",
            display_name
        ));
        return;
    }

    // Método 2: Remover paquete provisionado (evita reinstalación)
    let cmd2 = format!(
        "Get-AppxProvisionedPackage -Online | Where-Object {{ $_.PackageName -like '*{}*' }} | ForEach-Object {{ Remove-AppxProvisionedPackage -Online -PackageName $_.PackageName -ErrorAction SilentlyContinue 2>$null }}",
        package_name
    );
    let _ = run_powershell(&cmd2);
    std::thread::sleep(std::time::Duration::from_millis(300));
    if !is_app_installed(package_name) {
        print_success(&format!(
            "'{}' eliminada [Método: Remove-ProvisionedPackage]",
            display_name
        ));
        return;
    }

    // Método 3: Forzar remoción con Remove-AppxPackage directo por PackageFullName
    let cmd3 = format!(
        "Get-AppxPackage -AllUsers '*{}*' | ForEach-Object {{ Remove-AppxPackage -Package $_.PackageFullName -AllUsers -ErrorAction SilentlyContinue 2>$null }}",
        package_name
    );
    let _ = run_powershell(&cmd3);
    std::thread::sleep(std::time::Duration::from_millis(300));
    if !is_app_installed(package_name) {
        print_success(&format!(
            "'{}' eliminada [Método: PackageFullName]",
            display_name
        ));
        return;
    }

    // Método 4: Usar winget si está disponible
    let cmd4 = format!(
        "winget uninstall --name '*{}*' --silent --accept-source-agreements 2>$null",
        package_name
    );
    let _ = run_powershell(&cmd4);
    std::thread::sleep(std::time::Duration::from_millis(500));
    if !is_app_installed(package_name) {
        print_success(&format!("'{}' eliminada [Método: Winget]", display_name));
        return;
    }

    // Método 5: DISM
    let cmd5 = format!(
        "Get-AppxPackage -AllUsers '*{}*' | ForEach-Object {{ dism /Online /Remove-ProvisionedAppxPackage /PackageName:$($_.PackageFullName) 2>$null }}",
        package_name
    );
    let _ = run_powershell(&cmd5);
    std::thread::sleep(std::time::Duration::from_millis(500));
    if !is_app_installed(package_name) {
        print_success(&format!("'{}' eliminada [Método: DISM]", display_name));
        return;
    }

    // Ningún método funcionó
    print_error(&format!(
        "'{}' NO se pudo eliminar (app protegida del sistema)",
        display_name
    ));
}

/// Desactiva un servicio de Windows
pub fn disable_service(service_name: &str) {
    let cmd = format!(
        "Stop-Service -Name '{}' -Force -ErrorAction SilentlyContinue; Set-Service -Name '{}' -StartupType Disabled -ErrorAction SilentlyContinue",
        service_name, service_name
    );

    match run_powershell(&cmd) {
        Ok(_) => print_success(&format!("Servicio '{}' desactivado", service_name)),
        Err(e) if e.is_empty() => {
            print_success(&format!("Servicio '{}' desactivado", service_name))
        }
        Err(e) => print_error(&format!(
            "No se pudo desactivar '{}': {}",
            service_name,
            e.trim()
        )),
    }
}

/// Activa un servicio de Windows
pub fn enable_service(service_name: &str) {
    let cmd = format!(
        "Set-Service -Name '{}' -StartupType Automatic -ErrorAction SilentlyContinue; Start-Service -Name '{}' -ErrorAction SilentlyContinue",
        service_name, service_name
    );

    match run_powershell(&cmd) {
        Ok(_) => print_success(&format!("Servicio '{}' restaurado", service_name)),
        Err(e) if e.is_empty() => print_success(&format!("Servicio '{}' restaurado", service_name)),
        Err(e) => print_error(&format!(
            "No se pudo restaurar '{}': {}",
            service_name,
            e.trim()
        )),
    }
}

/// Desactiva una tarea programada
pub fn disable_scheduled_task(task_name: &str) {
    let cmd = format!(
        "Disable-ScheduledTask -TaskName '{}' -ErrorAction SilentlyContinue",
        task_name
    );

    match run_powershell(&cmd) {
        Ok(_) => print_success(&format!(
            "Tarea desactivada: {}",
            task_name.split('\\').last().unwrap_or(task_name)
        )),
        Err(e) if e.is_empty() => print_success(&format!(
            "Tarea desactivada: {}",
            task_name.split('\\').last().unwrap_or(task_name)
        )),
        Err(e) => print_error(&format!("No se pudo desactivar tarea: {}", e.trim())),
    }
}

/// Activa una tarea programada
pub fn enable_scheduled_task(task_name: &str) {
    let cmd = format!(
        "Enable-ScheduledTask -TaskName '{}' -ErrorAction SilentlyContinue",
        task_name
    );

    match run_powershell(&cmd) {
        Ok(_) => print_success(&format!(
            "Tarea restaurada: {}",
            task_name.split('\\').last().unwrap_or(task_name)
        )),
        Err(e) if e.is_empty() => print_success(&format!(
            "Tarea restaurada: {}",
            task_name.split('\\').last().unwrap_or(task_name)
        )),
        Err(e) => print_error(&format!("No se pudo restaurar tarea: {}", e.trim())),
    }
}

// === Funciones de impresión con colores ===

/// Imprime un encabezado de sección
pub fn print_header(text: &str) {
    println!("\n{}", format!("═══ {} ═══", text).cyan().bold());
}

/// Imprime un mensaje de éxito
pub fn print_success(text: &str) {
    println!("  {} {}", "✓".green().bold(), text);
}

/// Imprime un mensaje de error
pub fn print_error(text: &str) {
    println!("  {} {}", "✗".red().bold(), text);
}

/// Imprime un mensaje informativo
pub fn print_info(text: &str) {
    println!("  {} {}", "→".blue(), text);
}

/// Imprime un mensaje de advertencia
pub fn print_warning(text: &str) {
    println!("  {} {}", "⚠".yellow().bold(), text);
}

/// Verifica si se está ejecutando como administrador
pub fn is_admin() -> bool {
    match run_powershell(
        "([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)",
    ) {
        Ok(result) => result.trim().eq_ignore_ascii_case("true"),
        Err(_) => false,
    }
}

/// Muestra el banner de la aplicación
pub fn print_banner() {
    println!(
        "{}",
        r#"
 __      __ _        ___        _     _             _            
 \ \    / /(_) _ _  |   \  ___ | |__ | | ___  __ _ | |_  ___  _ _ 
  \ \/\/ / | || ' \ | |) |/ -_)| '_ \| |/ _ \/ _` ||  _|/ -_)| '_|
   \_/\_/  |_||_||_||___/ \___||_.__/|_|\___/\__,_| \__|\___||_|  
                                                            v2.0
"#
        .cyan()
        .bold()
    );
}
