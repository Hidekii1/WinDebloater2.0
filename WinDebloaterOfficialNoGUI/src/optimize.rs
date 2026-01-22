// Módulo para optimización de Windows
use crate::utils::{print_error, print_header, print_info, print_success, run_powershell};

pub fn run_optimization() {
    print_header("Optimización de Windows");

    // Desactivar animaciones visuales para mejor rendimiento
    print_info("Ajustando efectos visuales...");
    let visual_effects = r#"
        Set-ItemProperty -Path 'HKCU:\Control Panel\Desktop' -Name 'UserPreferencesMask' -Value ([byte[]](0x90,0x12,0x03,0x80,0x10,0x00,0x00,0x00)) -Force
        Set-ItemProperty -Path 'HKCU:\Control Panel\Desktop\WindowMetrics' -Name 'MinAnimate' -Value '0' -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'TaskbarAnimations' -Value 0 -Force
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects' -Name 'VisualFXSetting' -Value 2 -Force
    "#;
    match run_powershell(visual_effects) {
        Ok(_) => print_success("Efectos visuales optimizados"),
        Err(e) if e.is_empty() => print_success("Efectos visuales optimizados"),
        Err(e) => print_error(&format!("Error en efectos visuales: {}", e.trim())),
    }

    // Desactivar inicio rápido (puede causar problemas)
    print_info("Configurando opciones de energía...");
    match run_powershell("powercfg /hibernate off") {
        Ok(_) => print_success("Inicio rápido desactivado"),
        Err(e) if e.is_empty() => print_success("Inicio rápido desactivado"),
        Err(e) => print_error(&format!("Error en inicio rápido: {}", e.trim())),
    }

    // Configurar esquema de energía alto rendimiento
    match run_powershell("powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c") {
        Ok(_) => print_success("Plan de energía: Alto rendimiento"),
        Err(_) => print_info("Plan de alto rendimiento no disponible"),
    }

    // Limpiar archivos temporales
    print_info("Limpiando archivos temporales...");
    let clean_cmd = r#"
        Remove-Item -Path "$env:TEMP\*" -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item -Path "C:\Windows\Temp\*" -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item -Path "$env:LOCALAPPDATA\Microsoft\Windows\Explorer\thumbcache_*.db" -Force -ErrorAction SilentlyContinue
    "#;
    match run_powershell(clean_cmd) {
        Ok(_) => print_success("Archivos temporales eliminados"),
        Err(_) => print_success("Archivos temporales eliminados (algunos en uso)"),
    }

    // Optimizar prefetch
    print_info("Optimizando configuración del sistema...");
    let prefetch_cmd = "Set-ItemProperty -Path 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management\\PrefetchParameters' -Name 'EnablePrefetcher' -Value 3 -Force";
    match run_powershell(prefetch_cmd) {
        Ok(_) => print_success("Prefetch configurado"),
        Err(e) if e.is_empty() => print_success("Prefetch configurado"),
        Err(e) => print_error(&format!("Error en Prefetch: {}", e.trim())),
    }

    // Desactivar notificaciones de sugerencias
    let tips_cmd = "Set-ItemProperty -Path 'HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager' -Name 'SubscribedContent-338389Enabled' -Value 0 -Force";
    match run_powershell(tips_cmd) {
        Ok(_) => print_success("Sugerencias de Windows desactivadas"),
        Err(e) if e.is_empty() => print_success("Sugerencias de Windows desactivadas"),
        Err(_) => {}
    }

    println!("\n  Optimización completada.");
}
