use std::{env, fs};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

macro_rules! printerr {
    ($($expr:expr),*) => { writeln!(&mut std::io::stderr(), $($expr),*) }
}

fn main() {
    let current_dir = env::current_dir();
    if current_dir.is_err() { printerr!("build no current_dir"); }
    let current_dir = current_dir.unwrap();
    // let target_dir = os::get_env("CARGO_TARGET_DIR").unwrap();
    let out_dir = env::var("OUT_DIR");
    if out_dir.is_err() { printerr!("build no OUT_DIR"); }
    let out_dir = out_dir.unwrap();
    let mut bdw_dir = current_dir.clone();
    bdw_dir.push("src");
    bdw_dir.push("bdwgc");
    let mut configure_path = bdw_dir.clone();
    configure_path.push("configure");
    // println!("configure_path: {:?}", configure_path);
    // println!("out_dir: {:?}", out_dir);

    let mut cmd = Command::new(configure_path);
    cmd.current_dir(out_dir.clone());
    let cmd_output = cmd.output();
    if let Err(ref err) = cmd_output {
        printerr!("build no cmd_output: {:?}", err);
        printerr!("configure_path: {}", configure_path.display());
        printerr!("out_dir: {}", out_dir.display());
    }
    let cmd_output = cmd_output.unwrap();
    if !cmd_output.status.success() {
        printerr!("configure status: {}", cmd_output.status);
        printerr!("configure stdout: {}", String::from_utf8_lossy(&cmd_output.stdout));
        printerr!("configure stderr: {}", String::from_utf8_lossy(&cmd_output.stderr));
    }
    assert!(cmd_output.status.success());

    // let make_cflags = "CFLAGS= -DGC_DEBUG -DGC_ALWAYS_MULTITHREADED -DGC_DISCOVER_TASK_THREADS -DDEBUG_THREADS";
    let make_cflags = "CFLAGS= -DGC_DEBUG -DGC_ALWAYS_MULTITHREADED -DGC_DISCOVER_TASK_THREADS ";
    // let make_cflags = "CFLAGS= -DGC_ALWAYS_MULTITHREADED -DGC_DISCOVER_TASK_THREADS ";

    // TODO: record the `make_cflags` that we used into a file
    // somewhere in the out_dir as well, and if there's a future
    // mismatch, then do a `make clean` before the `make`.

    let mut cmd = Command::new("make");
    cmd.current_dir(out_dir.clone());
    cmd.arg(make_cflags);
    let cmd_output = cmd.output();
    if cmd_output.is_err() { printerr!("build make failed"); }
    let cmd_output = cmd_output.unwrap();
    if !cmd_output.status.success() {
        printerr!("make status: {}", cmd_output.status);
        printerr!("make stdout: {}", String::from_utf8_lossy(&cmd_output.stdout));
        printerr!("make stderr: {}", String::from_utf8_lossy(&cmd_output.stderr));
    }

    assert!(cmd_output.status.success());

    let mut libs_dir = PathBuf::from(out_dir);
    libs_dir.push(".libs");
    let mut libgc_a = libs_dir.clone();
    libgc_a.push("libgc.a");
    let libgc_a_metadata = fs::metadata(libgc_a);
    if libgc_a_metadata.is_err() { printerr!("build make libgc.a unfound."); }
    let libgc_a_metadata = libgc_a_metadata.unwrap();
    assert!(libgc_a_metadata.is_file());

    println!("cargo:rustc-link-lib=static=gc");
    println!("cargo:rustc-link-search=native={}", libs_dir.display());
}

