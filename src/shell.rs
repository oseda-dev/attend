use std::{error::Error, fs::{OpenOptions, create_dir_all}, io, path::Path};



pub fn mkdir_p(path: &Path) -> Result<(), Box<dyn Error>> {

    create_dir_all(path)?;
    Ok(())
}

pub fn touch(path: &Path) -> io::Result<()> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)?;
    Ok(())
}
