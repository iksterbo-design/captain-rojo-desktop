# Captain Rojo — Desktop

AI branding & ad-production desktop app for the Alienware 16X (Intel Core Ultra 9 275HX, 24-core, 32GB, NVIDIA RTX).

## Build & Run
```bash
# 1. Install Rust + Tauri CLI
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install tauri-cli

# 2. Install frontend deps
npm install

# 3. Dev (with auto-reload)
npm run dev

# 4. Build release
npm run build
```
Output: `src-tauri/target/release/captain-rojo.exe`

## Efficiency
- Parallel builds (`-Porg.gradle.parallel=true`, `-Pmax-workers=24`)
- GPU-accelerated ffmpeg encoding (NVENC/AMF/QSV)
- CPU pooling + lazy asset loading
