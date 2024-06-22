use super::basic::*;

/// 利用supervisorctl管理服务
pub fn netplan(sub_cmd: impl AsRef<str>) -> CommandOutput {
    let program = "/usr/sbin/netplan";
    let args = [sub_cmd.as_ref()];
    let title = format!("netplan_{}", sub_cmd.as_ref());
    run_command(program, args, &title).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apt_install_test() {
        init_log(1);
        let r = netplan("apply");
        println!("stdout: {}", r.stdout);
        println!("stderr: {}", r.stderr);
    }
}
