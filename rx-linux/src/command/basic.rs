pub use rx_core::log::*;
pub use rx_core::text::BoxResult;

use std::ffi::OsStr;
use std::fmt::Debug;
use std::io;
use std::io::Write;

use std::process::{Command, Output, Stdio};

pub type StrRef = dyn AsRef<str>;

// type StrRef = impl AsRef<str>; TODO: 报错, 参考 type_alias_impl_trait

// 统一管理, 方便验证路径
pub const SSHPASS: &str = "/usr/bin/sshpass";
pub const RSYNC: &str = "/usr/bin/rsync";
pub const SH: &str = "/usr/bin/sh";
pub const APT: &str = "/usr/bin/apt";
pub const SUDO: &str = "/usr/bin/sudo";

pub const SUPER_CTL: &str = "/usr/bin/supervisorctl";

/// 命令输出
#[derive(Debug, Default, Clone)]
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

/// 运行命令
pub fn run_command<I, S, T>(program: S, args: I, title: T) -> Option<CommandOutput>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr> + Debug,
    T: AsRef<str>,
{
    error!("program: {:?} '{:?}'", &program, &args);
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
