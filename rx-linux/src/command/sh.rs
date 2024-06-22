use super::basic::*;
use rx_core::sys::fs::to_string;
use std::path::Path;

/// 利用supervisorctl管理服务
pub fn run_sh(sh_path: impl AsRef<Path>) -> Option<CommandOutput> {
    let title = "SH".to_string();
    let sh_path = to_string(sh_path.as_ref());
    let args = [sh_path.as_ref()];
    run_command(SH, args, &title)
}
