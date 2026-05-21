use std::{
    error::Error, path::{Path, PathBuf}
};

/// Performs a full path expansion
/// 
/// # Arguments
/// 
/// - `path` (`&Path`) - Path containing special path values.
/// 
/// # Returns
/// 
/// - `Result<(), Box<dyn Error>>` - Ok(Path) on success, propogating error. The returned path will home values such as ~ expanded to the users home directory
/// 
pub fn expand_path(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let expanded = shellexpand::tilde(
        path.to_str()
            .ok_or_else(|| "Could not convert path to string")?,
    );

    Ok(expanded.into_owned().into())
}
