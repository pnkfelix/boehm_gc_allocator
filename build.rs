use std::{env, fs};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let current_dir = env::current_dir().unwrap();
    // let target_dir = os::get_env("CARGO_TARGET_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut bdw_dir = current_dir.clone();
    bdw_dir.push("src");
    bdw_dir.push("bdwgc");
    let mut configure_path = bdw_dir.clone();
    configure_path.push("configure");
    // println!("configure_path: {:?}", configure_path);
    // println!("out_dir: {:?}", out_dir);

    let mut cmd = Command::new(configure_path);
    cmd.current_dir(out_dir.clone());
    let cmd_output = cmd.output().unwrap();
    if !cmd_output.status.success() {
        writeln!(&mut std::io::stderr(), "configure status: {}", cmd_output.status);
        writeln!(&mut std::io::stderr(), "configure stdout: {}", String::from_utf8_lossy(&cmd_output.stdout));
        writeln!(&mut std::io::stderr(), "configure stderr: {}", String::from_utf8_lossy(&cmd_output.stderr));
    }
    assert!(cmd_output.status.success());

    let mut cmd = Command::new("make");
    cmd.current_dir(out_dir.clone());
    // cmd.arg("CFLAGS=-DGC_DEBUG -DGC_ALWAYS_MULTITHREADED -DGC_DISCOVER_TASK_THREADS -DDEBUG_THREADS");
    cmd.arg("CFLAGS=-DGC_DEBUG -DGC_ALWAYS_MULTITHREADED -DDEBUG_THREADS");
    let cmd_output = cmd.output().unwrap();
    if !cmd_output.status.success() {
        writeln!(&mut std::io::stderr(), "make status: {}", cmd_output.status);
        writeln!(&mut std::io::stderr(), "make stdout: {}", String::from_utf8_lossy(&cmd_output.stdout));
        writeln!(&mut std::io::stderr(), "make stderr: {}", String::from_utf8_lossy(&cmd_output.stderr));
    }
    assert!(cmd_output.status.success());

    let mut libs_dir = PathBuf::from(out_dir);
    libs_dir.push(".libs");
    let mut libgc_a = libs_dir.clone();
    libgc_a.push("libgc.a");
    assert!(fs::metadata(libgc_a).unwrap().is_file());

    println!("cargo:rustc-link-search=native={}", libs_dir.display());
}

