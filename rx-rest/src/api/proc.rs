use rx_core::sys::fs::to_string;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use super::common::*;

const SSHPASS: &str = "/usr/bin/sshpass";
const RSYNC: &str = "/usr/bin/rsync";
const SH: &str = "/usr/bin/sh";
const APT: &str = "/usr/bin/apt";
const SUDO: &str = "/usr/bin/sudo";

/// 命令输出
#[derive(Object, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CommandOutput {
    /// 运行成功还是失败
    pub success: bool,
    /// 返回状态码信息
    pub exit_status: String,
    /// 标准输出信息
    pub stdout: String,
    /// 标准错误信息
    pub stderr: String,
}

/// 服务命令
#[derive(Object, Debug, Default, Clone, Serialize, Deserialize)]
pub struct ServiceCmd {
    /// 服务命令迟延
    pub delay: i32,
    /// 命令输出信息
    pub command_output: Option<CommandOutput>,
}

/// 运行命令
pub fn run_command<I, S, T>(program: S, args: I, title: T) -> Option<CommandOutput>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr> + Debug,
    T: AsRef<str>,
{
    debug!("program: {:?} '{:?}'", &program, &args);
    let r = Command::new(program).args(args).output();
    proc_output(title, r)
}

/// 运行命令, 含有输入
pub fn run_command_inputs<I, S>(
    program: S,
    args: I,
    title: impl AsRef<str>,
    inputs: impl AsRef<str>,
) -> Option<CommandOutput>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start command");
    if let Some(stdin) = child.stdin.as_mut() {
        //sleep(Duration::from_secs(4));
        info!("password='{}'", inputs.as_ref());
        stdin
            .write_all(inputs.as_ref().as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output();
    proc_output(title, output)
}

fn proc_output(title: impl AsRef<str>, output_result: io::Result<Output>) -> Option<CommandOutput> {
    match output_result {
        Ok(output) => {
            let out = CommandOutput {
                success: output.status.success(),
                exit_status: output.status.to_string(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            };
            if out.success {
                trace!("run='{}' OK", title.as_ref());
            } else {
                error!(
                    "run='{}' exit_status='{}'",
                    title.as_ref(),
                    &out.exit_status
                );
                error!("run='{}' stdout='{}'", title.as_ref(), &out.stdout);
                error!("run='{}' stderr='{}'", title.as_ref(), &out.stderr);
            }
            Some(out)
        }
        Err(err) => {
            error!("run='{:?}' error='{}'", title.as_ref(), err.to_string());
            None
        }
    }
}

/// 利用supervisorctl管理服务
pub fn supervisorctl(
    _record: Json<ServiceCmd>,
    sub_cmd: impl AsRef<str>,
    service: impl AsRef<str>,
) -> Result<CodeResponse<ServiceCmd>> {
    let program = "/usr/bin/supervisorctl";
    let args = [sub_cmd.as_ref(), service.as_ref()];
    let title = format!(
        "supervisorctl_{:?}_{:?}",
        sub_cmd.as_ref(),
        service.as_ref()
    );
    let r = run_command(program, args, &title);
    to_resp(r)
}

pub trait AsRef1<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

// type StrRef = impl AsRef<str>; TODO: 报错, 参考 type_alias_impl_trait

/// 利用supervisorctl管理服务
pub fn rsync(
    opts: impl AsRef<str>,
    src: impl AsRef<str>,
    dst: impl AsRef<str>,
    password: Option<&str>,
) -> Option<CommandOutput> {
    let mut rsync_opts: Vec<_> = opts.as_ref().split(' ').collect();
    rsync_opts.append(&mut vec![src.as_ref(), dst.as_ref()]);

    let title = format!("rsync");
    if let Some(password) = password {
        let mut args = vec!["-p", password, RSYNC];
        args.append(&mut rsync_opts);
        run_command(SSHPASS, args, &title)
    } else {
        run_command(RSYNC, rsync_opts, &title)
    }
}

pub fn reboot(delay: i32) -> Option<CommandOutput> {
    let program = "shutdown -r now + 2";
    let args = ["-r", "now", "+", &delay.to_string()];
    let title = "reboot";
    run_command(program, args, &title)
}

/// 利用supervisorctl管理服务
pub fn run_sh(sh_path: impl AsRef<FsPath>) -> Option<CommandOutput> {
    let title = "SH".to_string();
    let sh_path = to_string(sh_path.as_ref());
    let args = [sh_path.as_ref()];
    run_command(SH, args, &title)
}

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

/// 利用supervisorctl管理服务
pub fn netplan(
    _record: Json<ServiceCmd>,
    sub_cmd: impl AsRef<str>,
) -> Result<CodeResponse<ServiceCmd>> {
    let program = "/usr/sbin/netplan";
    let args = [sub_cmd.as_ref()];
    let title = format!("netplan_{}", sub_cmd.as_ref());
    let r = run_command(program, args, &title);
    to_resp(r)
}

// 转换为响应
fn to_resp(r: Option<CommandOutput>) -> Result<CodeResponse<ServiceCmd>> {
    match r {
        None => Ok(CodeResponse::InternalServerError),
        Some(out) => {
            if out.success {
                let s = ServiceCmd {
                    delay: 0,
                    command_output: Some(out),
                };
                Ok(CodeResponse::Created(Json(s)))
            } else {
                Ok(CodeResponse::InternalServerError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsync_local() {
        init_log(1);
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
        init_log(1);
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

    #[test]
    fn apt_install_test() {
        init_log(1);
        let r = apt_install(&["jq", "qiv"], true).unwrap();
        println!("stdout: {}", r.stdout);
        println!("stderr: {}", r.stderr);
    }
}
