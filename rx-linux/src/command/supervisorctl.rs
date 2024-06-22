use super::basic::*;

/// 利用supervisorctl管理服务
pub fn supervisorctl(sub_cmd: impl AsRef<str>, service: impl AsRef<str>) -> Option<CommandOutput> {
    let program = SUPER_CTL;
    let args = [sub_cmd.as_ref(), service.as_ref()];
    let title = format!(
        "supervisorctl_{:?}_{:?}",
        sub_cmd.as_ref(),
        service.as_ref()
    );
    run_command(program, args, &title)
}
