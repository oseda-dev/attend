use std::{
    error::Error, fs::create_dir_all, path::{Path, PathBuf}
};

pub fn expand_path(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let expanded = shellexpand::tilde(
        path.to_str()
            .ok_or_else(|| "Could not convert path to string")?,
    );

    Ok(expanded.into_owned().into())
}

pub fn mkdir_p(path: &Path) -> Result<(), Box<dyn Error>> {

    create_dir_all(path)?;
    Ok(())
}
