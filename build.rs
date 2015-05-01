use std::process::Command;
use std::path::{Path, PathBuf};
use std::env;

fn main() {
    let out = Command::new("where").arg("cargo").output().unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    let rustc_dir = Path::new(s.trim()).parent().unwrap();
    let target = env::var("TARGET").unwrap();
    let gcc_dir = rustc_dir.join("rustlib").join(target).join("bin");

    let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());

    let old_path = env::var("PATH").unwrap();
    let new_path = {
        let mut paths = env::split_paths(&old_path).collect::<Vec<_>>();
        paths.push(gcc_dir.clone());
        env::join_paths(paths.iter()).unwrap()
    };
    // without adding gcc_dir to the own %PATH% of build-script
    // windres is addressed by it's full path and fails with the next error:
    // 'C:\Program' is not recognized as an internal or external command,
    //    operable program or batch file.
    env::set_var("PATH", &new_path);

    let mut cmd = Command::new("windres.exe");
    cmd.current_dir(&gcc_dir)
        .arg("-i").arg(&src.join("src").join("main.rc"))
        .arg("-o").arg(&dst.join("resources.o"));
    let res = cmd.output().unwrap();
    println!("windres status: {:?}", res.status);
    println!("windres stdout: {:?}", String::from_utf8(res.stdout).unwrap());
    println!("windres stderr: {:?}", String::from_utf8(res.stderr).unwrap());
    if !res.status.success() {
        panic!("Failed to compile resource file");
    }
    println!("cargo:rustc-flags= -C link-args=\"{}\"", &dst.join("resources.o").display());
}
