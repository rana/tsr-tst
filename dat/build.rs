use mcr::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    write_all_files("./src").unwrap();
}