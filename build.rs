fn main() {
    find_libgc();
}

fn find_libgc() {
    'found: loop {
        for d in &["/usr/local/lib"] {
            if std::fs::metadata(format!("{}/libgc.a", d)).is_ok() {
                println!("cargo:rustc-link-search=native={}", d);
                break 'found;
            }
        }
        panic!("no libgc found.");
    }
}
