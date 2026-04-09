use std::{
    error::Error, path::{Path, PathBuf}
};

pub fn expand_path(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let expanded = shellexpand::tilde(
        path.to_str()
            .ok_or_else(|| "Could not convert path to string")?,
    );

    Ok(expanded.into_owned().into())
}
