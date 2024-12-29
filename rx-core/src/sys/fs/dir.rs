use std::collections::HashSet;
use std::path::{Path, PathBuf};

use path_macro::path;
use tracing::{error, info};

use crate::sys::fs::{SortOrder, file_name, files_in};
use crate::text::AnyResult;

/// 两个目录中文件主干差集, 限定文件扩展名
pub fn dir_stem_diff(
    src_dir: &Path,
    dst_dir: &Path,
    src_ext: &str,
    dst_ext: &str,
) -> AnyResult<Vec<PathBuf>> {
    let src_files = files_in(src_dir, src_ext, SortOrder::None)?;
    let dst_files = files_in(dst_dir, dst_ext, SortOrder::None)?;
    let dst_stems: HashSet<_> = dst_files.iter().map(|f| f.file_stem().unwrap()).collect();

    let mut diff_files: Vec<_> = src_files
        .iter()
        .filter(|f| !dst_stems.contains(f.file_stem().unwrap()))
        .map(|f| f.to_path_buf())
        .collect();
    diff_files.sort();
    Ok(diff_files)
}

pub trait FileTranslator {
    fn translate(&self, src: &Path, dst: &Path) -> AnyResult<()>;
}

/// 目录文件翻译
pub struct DirTranslator {
    pub src_ext: String,
    pub dst_ext: String,
}

impl DirTranslator {
    pub fn translate_dir(
        &self,
        src_dir: &Path,
        dst_dir: &Path,
        translator: &impl FileTranslator,
    ) -> AnyResult<usize> {
        let diff_files = dir_stem_diff(src_dir, dst_dir, &self.src_ext, &self.dst_ext).unwrap();
        let total = diff_files.len();
        for src_file in diff_files {
            let dst_file = path!(dst_dir / file_name(src_file.with_extension("srt")));
            match translator.translate(&src_file, &dst_file) {
                Ok(_) => info!(
                    "translate: {} => {}",
                    src_file.display(),
                    dst_file.display()
                ),
                Err(_) => error!(
                    "translate: {} => {}",
                    src_file.display(),
                    dst_file.display()
                ),
            }
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;

    use tempfile::tempdir;

    use super::*;

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

    struct TestFileTranslator;

    impl FileTranslator for TestFileTranslator {
        fn translate(&self, src: &Path, dst: &Path) -> AnyResult<()> {
            let mut dst_file = File::create(dst)?;
            writeln!(dst_file, "Translated from {:?}", src)?;
            Ok(())
        }
    }

    #[test]
    fn test_dir_translator() {
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

        // 创建 DirTranslator 实例
        let dir_translator = DirTranslator {
            src_ext: "txt".to_string(),
            dst_ext: "md".to_string(),
        };

        // 调用 translate_dir 方法
        dir_translator
            .translate_dir(&src_dir, &dst_dir, &TestFileTranslator)
            .unwrap();

        // 验证翻译结果
        let translated_file = dst_dir.join("file3.srt");
        assert!(translated_file.exists());
        let content = fs::read_to_string(translated_file).unwrap();
        assert!(content.contains("Translated from"));
    }
}
