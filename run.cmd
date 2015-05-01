cargo clean
cargo build
rustc src\main.rs --crate-name manifest --crate-type bin -g --out-dir %~dp0target\debug --emit=dep-info,link -L dependency=%~dp0target\debug -L dependency=%~dp0target\debug\deps -C link-args="%~dp0target\debug\build\manifest-0b24ce90fdfaf1b3\out\resources.o"
%~dp0target\debug\manifest.exe
