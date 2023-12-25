use std::path::{Path, PathBuf};


pub fn get_testdata_path( filename : &str ) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join(filename)
}