/// Collection of shell wrappers
use std::{
    error::Error,
    fs::{OpenOptions, create_dir_all},
    io,
    path::Path,
};

/// Always makes a path, even if it already exists.
///
/// # Arguments
///
/// - `path` (`&Path`) - Path to create
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - Ok(()) on success, propogating error
///
pub fn mkdir_p(path: &Path) -> Result<(), Box<dyn Error>> {
    create_dir_all(path)?;
    Ok(())
}

/// Create a file, even if it already exists, but do not override it
///
/// # Arguments
///
/// - `path` (`&Path`) - Path to touch
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - Ok(()) on success, propogating error
///
pub fn touch(path: &Path) -> io::Result<()> {
    OpenOptions::new().create(true).write(true).open(path)?;
    Ok(())
}
