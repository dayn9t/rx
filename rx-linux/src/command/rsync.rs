use super::basic::*;

/// 利用supervisorctl管理服务
pub fn rsync(
    opts: impl AsRef<str>,
    src: impl AsRef<str>,
    dst: impl AsRef<str>,
    password: Option<&str>,
) -> Option<CommandOutput> {
    if let Some(password) = password {
        let args = [
            "-p",
            password,
            RSYNC,
            opts.as_ref(),
            src.as_ref(),
            dst.as_ref(),
        ];
        let title = format!("rsync_{:?}_{:?}_{:?}", args[0], args[1], args[2]);
        run_command(SSHPASS, args, &title)
    } else {
        let args = [opts.as_ref(), src.as_ref(), dst.as_ref()];
        let title = format!("rsync_{:?}_{:?}_{:?}", args[0], args[1], args[2]);
        run_command(RSYNC, args, &title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsync_local() {
        let r = rsync(
            "-avc",
            "/opt/howell/iws/v0.9/",
            "/opt/howell/iws/v0.8/",
            None,
        )
        .unwrap();
        println!("stdout: {}", r.stdout);
        println!("stderr: {}", r.stderr);
    }

    #[test]
    fn rsync_remote() {
        let r = rsync(
            "-avc",
            "howell@101.91.231.243:/opt/howell/.meta/updater",
            "/opt/howell/.meta/updater",
            Some("Howell.net.cn1409"),
        )
        .unwrap();
        println!("stdout: {}", r.stdout);
        println!("stderr: {}", r.stderr);
    }
}
