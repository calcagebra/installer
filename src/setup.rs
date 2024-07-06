use std::{
    env::var,
    fs::{copy, create_dir_all, metadata, remove_file, set_permissions, write, Permissions},
    os::unix::prelude::PermissionsExt,
};

pub fn setup(os: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    match os {
        "windows" => {
            if metadata("C:\\bin\\calcagebra").is_err() {
                create_dir_all("C:\\bin\\calcagebra")?;
            }

            copy("calcagebra.exe", "C:\\bin\\calcagebra\\calcup.exe")?;
            remove_file("calcup.exe")?;
        }
        "macos" | "linux" => {
            let home = var("HOME")?;
            if metadata(format!("{home}/.calcagebra")).is_err() {
                create_dir_all(format!("{home}/.calcagebra/bin"))?;
                set_permissions(
                    format!("{home}/.calcagebra/bin"),
                    Permissions::from_mode(0o770),
                )?;
            }

            write(format!("{home}/.calcagebra/env"), include_str!("./env.sh"))?;

            if metadata("./calcup").is_ok() {
                set_permissions("./calcup", Permissions::from_mode(0o770))?;
                copy("calcup", format!("{home}/.calcagebra/bin/calcup"))?;
                remove_file("calcup")?;
            }
        }
        other => panic!("Unsupported target arch: `{other}`"),
    };

    Ok(())
}
