use std::ffi::OsStr;
use std::process::Command;

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
    match r {
        Ok(output) => {
            let out = CommandOutput {
                success: output.status.success(),
                exit_status: output.status.to_string(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            };
            if !out.success {
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
pub fn supervisorctl<S>(
    _record: Json<ServiceCmd>,
    sub_cmd: S,
    service: S,
) -> Result<CodeResponse<ServiceCmd>>
where
    S: AsRef<str>,
{
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

/// 利用supervisorctl管理服务
pub fn netplan<S>(_record: Json<ServiceCmd>, sub_cmd: S) -> Result<CodeResponse<ServiceCmd>>
where
    S: AsRef<str>,
{
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
