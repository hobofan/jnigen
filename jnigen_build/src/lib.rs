extern crate jnigen_shared;

use jnigen_shared::helpers;
use std::path::Path;

pub fn prepare_build() {
    let outdir_hint = helpers::get_out_dir_hint();
    if let Some(outdir_hint) = outdir_hint {
        let structurefile_path = Path::new(&outdir_hint).join("jni-structure.json");
        if structurefile_path.exists() {
            std::fs::remove_file(structurefile_path).unwrap();
        }
    }
}
