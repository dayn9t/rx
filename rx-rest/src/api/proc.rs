use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use super::common::*;

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
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    T: AsRef<str>,
{
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
                info!("run='{}' OK", title.as_ref());
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

const SSHPASS: &str = "/usr/bin/sshpass";
const RSYNC: &str = "/usr/bin/rsync";

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
}
