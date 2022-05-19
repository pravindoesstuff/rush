pub fn redirect(
    destination: &str,
    child: &mut Option<std::process::Child>,
) -> Result<(), std::io::Error> {
    let mut dest_file = match std::fs::File::create(destination) {
        Ok(fd) => fd,
        Err(e) => {
            return Err(e);
        }
    };
    if let Some(child) = child {
        if let Err(e) = std::io::copy(child.stdout.as_mut().unwrap(), &mut dest_file) {
            return Err(e);
        }
    }
    Ok(())
}
