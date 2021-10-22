use ssh::{ConfigMap, Session};
pub use wezterm_ssh as ssh;

pub fn connect_to_server() -> Session {
    let map = ConfigMap::new();

    // Will panic, but we don't care for compilation failures
    let (session, _) = Session::connect(map).unwrap();

    session
}
