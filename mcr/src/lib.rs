use proc_macro2::TokenStream;
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tar::{Archive, EntryType};
use tsr_tst_dat_xz::Dat;
use xz2::read::XzDecoder;

pub fn write_all_files(dir: &str) -> std::io::Result<()> {
    let pth = Path::new(dir);
    fs::create_dir_all(pth)?;
    write_one_fle(emit_fle_lib(), &pth.join("lib.rs"))?;
    let pth_duk = pth.join("duk");
    fs::create_dir_all(&pth_duk)?;
    write_one_fle(emit_fle_duk_mod(), &pth_duk.join("mod.rs"))?;
    write_one_fle(emit_fle_duk_goog(&pth_duk), &pth_duk.join("goog.rs"))?;
    Ok(())
}

pub fn write_one_fle(fle_stm: TokenStream, fle_pth: &PathBuf) -> std::io::Result<()> {
    let fle = syn::parse_file(fle_stm.to_string().as_str()).unwrap();
    let fmt = prettyplease::unparse(&fle);
    fs::write(&fle_pth, fmt)
}

/// `emit_fle_lib` emits a `lib` file.
pub fn emit_fle_lib() -> TokenStream {
    let mut fle_stm = TokenStream::new();

    fle_stm.extend(quote! {
        /// `duk` module provides financial data from Dukascopy.
        pub mod duk;
    });

    return fle_stm;
}

/// `emit_fle_duk` emits a `duk` file.
pub fn emit_fle_duk_mod() -> TokenStream {
    let mut fle_stm = TokenStream::new();

    fle_stm.extend(quote! {
        /// `goog` module provides Google stock data.
        pub mod goog;
    });

    return fle_stm;
}

/// `emit_fle_goog` emits a `goog` file.
pub fn emit_fle_duk_goog(pth_duk: &PathBuf) -> TokenStream {
    // Create a `duk/bin/` directory
    // Binary files will be written to the directory
    let pth_duk_bin = pth_duk.join("bin");
    fs::create_dir_all(&pth_duk_bin).unwrap();

    // Create a rust token file
    let mut fle_stm = TokenStream::new();

    // Open the compressed archive of CSV data
    let duk_tar_xz = Dat::get("duk.tar.xz").unwrap();
    let duk_tar_dec = XzDecoder::new(duk_tar_xz.data.as_ref());
    let mut duk_tar = Archive::new(duk_tar_dec);

    // Cycle through each of the CSV files
    for tar_itm in duk_tar.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let tar_itm = tar_itm.unwrap();

        let tar_itm_pth = tar_itm.header().path().unwrap();
        let fle_name = tar_itm_pth.file_stem().unwrap();
        println!(
            "tar_itm_pth: {:?} {:?} {:?} {} {}",
            tar_itm_pth,
            tar_itm_pth.file_stem().unwrap(),
            tar_itm.header().entry_type().is_file(),
            tar_itm_pth.is_file(),
            tar_itm_pth.ends_with(".csv")
        );
        if tar_itm.header().entry_type().is_file() {
            // Entry looks like:
            //  "duk/goog/2020_02_04.csv"
            // println!("tar_itm_pth: {:?}", tar_itm_pth);
            let fle_name = tar_itm_pth.file_stem().unwrap();
            println!("fle_name: {:?}", fle_name);
            // let fle_tok = TokenStream::from_str(fle_name.to_str().unwrap()).unwrap();
            // fle_stm.extend(quote! {
            //     pub const #fle_tok: usize = 4;
            // });

            // TODO: DELETE
            break;
        }

        // // files implement the Read trait
        // let mut s = String::new();
        // file.read_to_string(&mut s).unwrap();
        // println!("{}", s);
    }

    fle_stm.extend(quote! {
        pub const CAV: usize = 4;
    });

    return fle_stm;
}

#[cfg(test)]
pub mod tst {
    use super::*;

    #[test]
    pub fn it_works() {
        write_all_files("./DELETE").unwrap();
    }
}
