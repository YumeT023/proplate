use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

pub fn copy_directory(src: &Path, dst: &Path) -> Result<(), Error> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidInput,
                "File does not have a valid filename",
            )
        })?;
        let dst_path = dst.join(file_name);
        if path.is_dir() {
            fs::create_dir(&dst_path)?;
            copy_directory(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
        }
    }
    Ok(())
}
