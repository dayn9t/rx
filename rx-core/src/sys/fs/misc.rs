pub use std::fs::File;
use std::fs::{self};
pub use std::io::*;
use std::os::unix::fs::symlink;
pub use std::path::{Path, PathBuf};

use chrono::prelude::*;

/// 路径连接
pub fn join<P1, P2>(p1: &P1, p2: &P2) -> PathBuf
where
    P1: AsRef<Path> + ?Sized,
    P2: AsRef<Path> + ?Sized,
{
    let mut p = p1.as_ref().to_owned();
    p.push(p2.as_ref());
    p
}

/// 删除路径（文件/目录）
pub fn remove(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if path.exists() {
        if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }
    } else {
        Ok(())
    }
}

/// 重建符号链接, 如果目标已经存在则删除
pub fn relink(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    let dst = dst.as_ref();
    if dst.is_symlink() {
        fs::remove_file(dst)?;
    }
    symlink(src, dst)
}

/// 路径替换
pub fn path_replace(path: &Path, src: &str, dst: &str) -> PathBuf {
    let path_str = path.to_str().unwrap();
    let new_path_str = path_str.replace(src, dst);
    PathBuf::from(new_path_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    const A_JSON: &str = "/tmp/a.json";

    #[test]
    fn join_works() {
        let s1 = "/usr";
        let s2 = "bin";

        let p1 = PathBuf::from("/usr/bin");
        let p2 = join(&s1, &s2);
        let _p3 = join(&"/usr", &"bin");
        let p3 = join(&s1, &"bin");

        assert_eq!(p1, p2);
        assert_eq!(p1, p3);
    }

    #[test]
    fn link_works() {
        let src = "/home";
        let dst = "/tmp/home";
        relink(src, dst).unwrap();
    }

    #[test]
    fn test_path_replace() {
        let original_path = Path::new("/path/to/snap/directory");
        let expected_path = Path::new("/path/to/img/directory");
        let result_path = path_replace(&original_path, "snap", "img");
        assert_eq!(result_path, expected_path);

        let original_path = Path::new("/snap/path/to/snap/directory");
        let expected_path = Path::new("/img/path/to/img/directory");
        let result_path = path_replace(&original_path, "snap", "img");
        assert_eq!(result_path, expected_path);

        let original_path = Path::new("/path/to/directory");
        let expected_path = Path::new("/path/to/directory");
        let result_path = path_replace(&original_path, "snap", "img");
        assert_eq!(result_path, expected_path);
    }
}
