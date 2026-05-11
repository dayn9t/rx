use crate::command::basic::*;

pub fn reboot(delay: i32) -> Option<CommandOutput> {
    let program = "/usr/sbin/shutdown";
    let args = ["-r", "now", "+", &delay.to_string()];
    let title = "reboot";
    run_command(program, args, title)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reboot() {}
}
