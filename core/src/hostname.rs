use nix::unistd;

pub fn get_hostname() -> Result<String, String> {
    unistd::gethostname()
        .map_err(|_| "Failed getting hostname...".to_owned())?
        .into_string()
        .map_err(|_| "Hostname wasn't valid UTF-8...".to_owned())
}
