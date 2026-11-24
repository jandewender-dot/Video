// Kein Konsolenfenster unter Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use tauri::ipc::{InvokeBody, Request, Response};

/// Nur Excel-Dateien lesen/schreiben – das Dashboard braucht nichts anderes.
fn excel_pfad(pfad: &str, nur_xlsx: bool) -> Result<&Path, String> {
    let p = Path::new(pfad);
    let ext = p.extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase()).unwrap_or_default();
    let erlaubt = if nur_xlsx { ext == "xlsx" } else { matches!(ext.as_str(), "xlsx" | "xlsm" | "xls" | "xlsb") };
    if !p.is_absolute() || !erlaubt {
        return Err(format!("Nicht erlaubter Dateipfad: {}", pfad));
    }
    Ok(p)
}

/// ASI_Stand.xlsx an den gewählten Ort schreiben. Inhalt als Rohdaten, Pfad im Header „x-pfad“ (URL-kodiert).
#[tauri::command]
fn datei_schreiben(request: Request<'_>) -> Result<(), String> {
    let InvokeBody::Raw(daten) = request.body() else {
        return Err("Keine Dateidaten erhalten".into());
    };
    let kodiert = request
        .headers()
        .get("x-pfad")
        .ok_or("Kein Speicherpfad angegeben")?
        .to_str()
        .map_err(|e| e.to_string())?;
    let pfad = percent_encoding::percent_decode_str(kodiert)
        .decode_utf8()
        .map_err(|e| e.to_string())?
        .to_string();
    let p = excel_pfad(&pfad, true)?;
    std::fs::write(p, daten).map_err(|e| e.to_string())
}

/// Excel-Datei vom gemerkten oder gewählten Ort lesen.
#[tauri::command]
fn datei_lesen(pfad: String) -> Result<Response, String> {
    let p = excel_pfad(&pfad, false)?;
    std::fs::read(p).map(Response::new).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![datei_schreiben, datei_lesen])
        .run(tauri::generate_context!())
        .expect("ASI-Dashboard konnte nicht gestartet werden");
}
