pub trait ResultExt<T, E> {
    fn ok_or_log(self, msg: &str) -> Option<T>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn ok_or_log(self, msg: &str) -> Option<T> {
        if let Ok(v) = self {
            Some(v)
        } else {
            weblog::console_error!(msg);
            None
        }
    }
}
