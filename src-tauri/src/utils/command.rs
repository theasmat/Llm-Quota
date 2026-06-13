use std::process::Command as StdCommand;
use tokio::process::Command as TokioCommand;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub trait CommandExtWrapper {
    ///  Windows  CREATE_NO_WINDOW ，
    fn creation_flags_windows(&mut self) -> &mut Self;
}

impl CommandExtWrapper for StdCommand {
    fn creation_flags_windows(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        self.creation_flags(CREATE_NO_WINDOW);

        self
    }
}

impl CommandExtWrapper for TokioCommand {
    fn creation_flags_windows(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        self.creation_flags(CREATE_NO_WINDOW);

        self
    }
}
