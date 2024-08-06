use crate::sys::fs::files_in;
use crate::text::BoxResult;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// 两个目录中文件主干差集, 限定文件扩展名
pub fn dir_stem_diff(
    src_dir: &Path,
    dst_dir: &Path,
    src_ext: &str,
    dst_ext: &str,
) -> BoxResult<Vec<PathBuf>> {
    let src_files = files_in(src_dir, src_ext)?;
    let dst_files = files_in(dst_dir, dst_ext)?;
    let dst_stems: HashSet<_> = dst_files.iter().map(|f| f.file_stem().unwrap()).collect();

    let diff_files = src_files
        .iter()
        .filter(|f| !dst_stems.contains(f.file_stem().unwrap()))
        .map(|f| f.to_path_buf())
        .collect();
    Ok(diff_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_dir_stem_diff() {
        // 创建临时目录
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src");
        let dst_dir = dir.path().join("dst");

        // 创建源目录和目标目录
        fs::create_dir_all(&src_dir).unwrap();
        fs::create_dir_all(&dst_dir).unwrap();

        // 在源目录中创建文件
        File::create(src_dir.join("file1.txt")).unwrap();
        File::create(src_dir.join("file2.txt")).unwrap();
        File::create(src_dir.join("file3.txt")).unwrap();

        // 在目标目录中创建文件
        File::create(dst_dir.join("file1.md")).unwrap();
        File::create(dst_dir.join("file2.md")).unwrap();

        // 调用 dir_stem_diff 函数
        let diff_files = dir_stem_diff(&src_dir, &dst_dir, "txt", "md").unwrap();

        // 验证返回的差集文件列表
        let diff_file_names: HashSet<_> = diff_files
            .iter()
            .map(|f| f.file_name().unwrap().to_str().unwrap())
            .collect();
        assert!(diff_file_names.contains("file3.txt"));
        assert!(!diff_file_names.contains("file1.txt"));
        assert!(!diff_file_names.contains("file2.txt"));
    }
}
