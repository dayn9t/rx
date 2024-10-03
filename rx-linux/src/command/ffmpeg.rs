use super::basic::*;
/// 利用supervisorctl管理服务
pub fn ffmpeg(args: impl AsRef<str>) -> Option<CommandOutput> {
    let program = FFMPEG;
    let args = args.as_ref().split(' ').collect::<Vec<&str>>();
    run_command(program, args, "FFMPEG")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffmpeg() {
        let args = "-version";
        let output = ffmpeg(args);
        assert!(output.is_some());
        let output = output.unwrap();
        assert!(output.success);
        assert!(output.stdout.contains("ffmpeg version"));
    }
}
