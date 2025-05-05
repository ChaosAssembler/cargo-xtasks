use std::{
    fs::{File, create_dir_all},
    io,
    path::Path,
};

use anyhow::bail;

pub(crate) fn copy_new<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    io::copy(&mut File::open(from)?, &mut File::create_new(to)?)
}

fn _copy_dir_files_filtered<P, Q, F>(
    from: P,
    to: Q,
    file_filter: &mut F,
    sup_dir_created: &mut bool,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(&Path) -> bool,
{
    let from = from.as_ref();
    let to = to.as_ref();

    let dir_created = if *sup_dir_created {
        &mut false
    } else {
        sup_dir_created
    };

    for dir_entry in from.read_dir()? {
        let dir_entry = dir_entry?;
        let file_type = dir_entry.file_type()?;
        let from_entry = dir_entry.path();
        let to_entry = to.join(from_entry.strip_prefix(from)?);

        if file_type.is_dir() {
            _copy_dir_files_filtered(from_entry, to_entry, &mut *file_filter, dir_created)?;
        } else if file_type.is_file() {
            if file_filter(&from_entry) {
                if !*dir_created {
                    create_dir_all(to)?;
                    *dir_created = true;
                }
                copy_new(from_entry, to_entry)?;
            }
        } else {
            bail!("Unsupported file type: {}", dir_entry.path().display())
        }
    }
    Ok(())
}

pub(crate) fn copy_dir_files_filtered<P, Q, F>(
    from: P,
    to: Q,
    mut file_filter: F,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(&Path) -> bool,
{
    _copy_dir_files_filtered(from, to, &mut file_filter, &mut false)
}
