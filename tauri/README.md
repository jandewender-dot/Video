# ASI-Dashboard – Desktop-Fassung (Tauri 2)

Portable Windows-EXE ohne Installer. Inhalt ist `web/index.html` (= gebaute `ASI_Dashboard_Leitstand_Bereiche.html`).

Bauen (unter Linux, Ziel Windows):

```bash
rustup target add x86_64-pc-windows-gnu
apt-get install -y gcc-mingw-w64-x86-64
cargo build --release --target x86_64-pc-windows-gnu
```

Ergebnis: `target/x86_64-pc-windows-gnu/release/asi-dashboard.exe` plus `WebView2Loader.dll` im selben Ordner.
