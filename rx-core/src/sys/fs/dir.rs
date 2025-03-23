use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use path_macro::path;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, fs};
use tracing::{debug, error, info};

use crate::sys::fs::{SortOrder, file_name, files_in};
use crate::text::AnyResult;

/// 获取当前可执行文件所在目录
pub fn current_exe_dir() -> PathBuf {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
}

/// 获取路径的指定代祖先
pub fn ancestor_nth(p: &Path, n: usize) -> &Path {
    p.ancestors().nth(n).unwrap()
}

/// 获取当前可执行文件祖先目录
pub fn current_exe_ancestor_nth(n: usize) -> PathBuf {
    ancestor_nth(&env::current_exe().unwrap(), n).to_path_buf()
}

/// 目录文件转换为字符串, 保证以＂/＂结尾
pub fn to_dir_string(path: impl AsRef<Path>) -> String {
    let mut path_str = path.as_ref().to_string_lossy().into_owned();
    if !path_str.ends_with("/") {
        path_str.push('/');
    }
    path_str
}

/// 配置目录
pub fn config_dir_of(name: impl AsRef<str>) -> PathBuf {
    dirs::config_dir().unwrap().join(&name.as_ref())
}

/// 创建上级目录，幂等
pub fn make_parent(path: impl AsRef<Path>) -> std::io::Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::NotFound, "parent not found"))
    }
}

/// 确保目录存在，不存在则建立
pub fn ensure_dir_exist(path: impl AsRef<Path>) -> std::io::Result<()> {
    let p = path.as_ref();
    if !p.exists() {
        fs::create_dir_all(&p)?;
    }
    if p.is_dir() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::AlreadyExists, "path not a dir!"))
    }
}

/// 遍历目录访问文件
pub fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry.path());
            }
        }
    }
    Ok(())
}

/// 遍历目录
pub fn visit_dir(dir: impl AsRef<Path>, cb: &mut dyn FnMut(&Path)) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        cb(&entry?.path());
    }
    Ok(())
}

/// 获取目录中的目录
pub fn dirs_in(dir: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    visit_dir(dir.as_ref(), &mut |p: &Path| {
        if p.is_dir() {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 获取目录中目录名
pub fn dir_names_in(dir: impl AsRef<Path>) -> std::io::Result<Vec<String>> {
    let v = dirs_in(dir)?;
    let v: Vec<_> = v.iter().map(|p| file_name(p)).collect();
    Ok(v)
}

/// 查找MTP设备目录
pub fn mtp_dirs() -> std::io::Result<Vec<PathBuf>> {
    dirs_in(&"/run/user/1000/gvfs")
}

/// 查找第一个目录(广度优先)
pub fn find_first_dir(
    dir: impl AsRef<Path>,
    dir_name: impl AsRef<str>,
) -> std::io::Result<PathBuf> {
    let mut dirs = Vec::new();

    for entry in fs::read_dir(dir.as_ref())? {
        let p = entry?.path();
        if p.is_dir() {
            if file_name(&p) == dir_name.as_ref() {
                return Ok(p);
            } else {
                dirs.push(p);
            }
        }
    }

    for dir in dirs {
        let r = find_first_dir(&dir, dir_name.as_ref());
        if r.is_ok() {
            return r;
        }
    }
    Err(Error::from(ErrorKind::NotFound))
}

/// 把一个目录的内容合并到另一个目录, 子目录不替换
pub fn merge_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !src.is_dir() {
        return Err(Error::new(ErrorKind::Other, "Source is not a directory"));
    }

    if !dst.exists() {
        fs::create_dir_all(dst)?;
    } else if !dst.is_dir() {
        return Err(Error::new(
            ErrorKind::Other,
            "Destination is not a directory",
        ));
    }

    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let src_path = entry.path();
        let dst_path = dst.join(src_path.file_name().unwrap());

        if src_path.is_dir() {
            merge_dir(&src_path, &dst_path)?;
        } else {
            debug!("Copy {:?} => {:?}", src_path, dst_path);
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// 复制目录树
pub fn copy_tree(src_dir: &Path, dst_dir: &Path) -> fs_extra::error::Result<u64> {
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    let paths_to_copy = vec![src_dir];
    fs::create_dir_all(dst_dir.parent().unwrap()).unwrap();
    copy_items(&paths_to_copy, dst_dir, &options)
}

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

/// 在路径的各个部分里查找指定路径
pub fn find_in_parts(folder: &Path, sub_path: &str) -> Option<PathBuf> {
    let mut folder = folder.canonicalize().ok()?;

    loop {
        let path = folder.join(sub_path);
        if path.exists() {
            return Some(path);
        }
        if let Some(parent) = folder.parent() {
            folder = parent.to_path_buf();
        } else {
            break;
        }
    }

    None
}
#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;

    use super::*;
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

    #[test]
    fn test_find_in_parts() {
        let tmp = tempdir().unwrap();
        let start_path = path!(tmp / "a" / "b");
        fs::create_dir_all(&start_path).unwrap();

        let a = path!(tmp / "a" / "a.txt");
        let b = path!(tmp / "a" / "b" / "b.txt");
        File::create(&a).unwrap();
        File::create(&b).unwrap();

        let p = find_in_parts(&start_path, "a.txt");
        assert_eq!(p, Some(a.to_owned()));

        let p = find_in_parts(&start_path, "b.txt");
        assert_eq!(p, Some(b.to_owned()));

        let p = find_in_parts(&start_path, "c.txt");
        assert_eq!(p, None);
    }
}
