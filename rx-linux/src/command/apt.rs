use crate::command::basic::*;

/// APT安装包
pub fn apt_install(pkgs: &[&str], sudo: bool) -> Option<CommandOutput> {
    let title = "APT".to_string();
    if sudo {
        let mut args = vec![APT, "install", "-y"];
        args.extend(pkgs.iter());
        run_command(SUDO, args, &title)
    } else {
        let mut args = vec!["install", "-y"];
        args.extend(pkgs.iter());
        run_command(APT, args, &title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apt_install_test() {
        let r = apt_install(&["jq", "qiv"], true).unwrap();
        println!("stdout: {}", r.stdout);
        println!("stderr: {}", r.stderr);
    }
}
