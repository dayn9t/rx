use chrono::prelude::*;
use std::env;
use std::ffi::OsStr;
pub use std::fs::File;
use std::fs::{self, DirEntry};
pub use std::io::*;
use std::os::unix::fs::symlink;
pub use std::path::{Path, PathBuf};

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

/// 时间转化为文件
pub fn time_to_file(dt: &DateTime<Local>, ext: &str) -> String {
    let filename = dt.format("%Y-%m-%d/%H-%M-%S%.3f.").to_string() + ext;
    filename
}

/// 当前时间转化为文件
pub fn now_to_file(ext: &str) -> String {
    let dt = Local::now();
    time_to_file(&dt, ext)
}

/// 文件作为字符串访问
pub fn to_str<P>(p: &P) -> &str
where
    P: AsRef<Path> + ?Sized,
{
    p.as_ref().to_str().unwrap()
}

/// 文件作为字符串访问
pub fn to_string(p: impl AsRef<Path>) -> String {
    p.as_ref().to_str().unwrap().to_owned()
}

/// 获取文件名
pub fn file_name(p: impl AsRef<Path>) -> String {
    p.as_ref().file_name().unwrap().to_str().unwrap().to_owned()
}

/// 获取扩展名，不包括＂.＂，比如＂jpg"，而不是".jpg"
pub fn file_ext(p: impl AsRef<Path>) -> String {
    p.as_ref().extension().unwrap().to_str().unwrap().to_owned()
}

/// 获取主干文件名(去掉扩展名)
pub fn file_stem(p: impl AsRef<Path>) -> String {
    p.as_ref().file_stem().unwrap().to_str().unwrap().to_owned()
}

/// 在文件名后面追加后缀
pub fn file_name_append(p: impl AsRef<Path>, s: &str) -> PathBuf {
    let stem = file_stem(p.as_ref());
    let ext = file_ext(p.as_ref());
    let name = format!("{}{}.{}", stem, s, ext);
    let parent = p.as_ref().parent().unwrap();
    parent.join(&name)
}

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

/// 配置目录
pub fn config_dir_of(name: impl AsRef<str>) -> PathBuf {
    dirs::config_dir().unwrap().join(&name.as_ref())
}

/// 创建上级目录，幂等
pub fn make_parent(path: impl AsRef<Path>) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::NotFound, "parent not found"))
    }
}

/// 确保目录存在，不存在则建立
pub fn ensure_dir_exist(path: impl AsRef<Path>) -> Result<()> {
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

/// 遍历目录访问文件
pub fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) -> Result<()> {
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

/// 获取递归目录中指定扩展名文件
pub fn find_file_by_ext(dir: impl AsRef<Path>, ext: impl AsRef<str>) -> Result<Vec<PathBuf>> {
    let ext = Some(OsStr::new(ext.as_ref()));
    let mut vec = Vec::new();
    visit_dirs(dir.as_ref(), &mut |p: &Path| {
        if p.is_file() && p.extension() == ext {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 获取递归目录中指定名称文件
pub fn find_file_by_name(dir: impl AsRef<Path>, name: impl AsRef<str>) -> Result<Vec<PathBuf>> {
    let name = Some(OsStr::new(name.as_ref()));
    let mut vec = Vec::new();
    visit_dirs(dir.as_ref(), &mut |p: &Path| {
        if p.is_file() && p.file_name() == name {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 遍历目录
pub fn visit_dir(dir: impl AsRef<Path>, cb: &mut dyn FnMut(&Path)) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        cb(&entry?.path());
    }
    Ok(())
}

/// 判断路径是有有扩展名
pub fn has_extension(_dir_entry: &DirEntry, _ext: impl AsRef<str>) -> bool {
    false
}

/// 获取目录中的目录
pub fn dirs_in(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    visit_dir(dir.as_ref(), &mut |p: &Path| {
        if p.is_dir() {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 获取目录中目录名
pub fn dir_names_in(dir: impl AsRef<Path>) -> Result<Vec<String>> {
    let v = dirs_in(dir)?;
    let v: Vec<_> = v.iter().map(|p| file_name(p)).collect();
    Ok(v)
}

/// 查找MTP设备目录
pub fn mtp_dirs() -> Result<Vec<PathBuf>> {
    dirs_in(&"/run/user/1000/gvfs")
}

/// 查找第一个目录(广度优先)
pub fn find_first_dir(dir: impl AsRef<Path>, dir_name: impl AsRef<str>) -> Result<PathBuf> {
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

/// 获取目录中文件
pub fn files_in(dir: impl AsRef<Path>, ext: impl AsRef<str>) -> Result<Vec<PathBuf>> {
    let ext = Some(OsStr::new(ext.as_ref()));
    let mut vec = Vec::new();
    visit_dir(dir, &mut |p: &Path| {
        if p.is_file() && p.extension() == ext {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 获取目录中文件名
pub fn file_names_in(dir: impl AsRef<Path>, ext: impl AsRef<str>) -> Result<Vec<String>> {
    let v = files_in(dir, ext)?;
    let v: Vec<_> = v.iter().map(|p| file_name(p)).collect();
    Ok(v)
}

/// 获取目录中文件名主干(去掉扩展名)
pub fn file_stems_in(dir: impl AsRef<Path>, ext: impl AsRef<str>) -> Result<Vec<String>> {
    let v = files_in(dir, ext)?;
    let v: Vec<_> = v.iter().map(|p| file_stem(p)).collect();
    Ok(v)
}

/// 合并目录内所有文件到一个文件
pub fn combine_files_in(src_dir: &Path, dst_file: &Path, ext: &str) -> Result<()> {
    let mut files = files_in(&src_dir, &ext)?;
    files.sort();
    combine_files(&files, dst_file)
}

/// 合并文件集合到一个文件
pub fn combine_files(src_files: &Vec<PathBuf>, dst_file: &Path) -> Result<()> {
    make_parent(&dst_file)?;
    let mut dst_file = File::create(dst_file)?;

    for file in src_files {
        println!("open: {:?}", file);
        let mut file = File::open(file)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        dst_file.write_all(&buf)?;
    }
    Ok(())
}

/// 重建符号链接, 如果目标已经存在则删除
pub fn relink(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    let dst = dst.as_ref();
    if dst.is_symlink() {
        fs::remove_file(dst).unwrap();
    }
    symlink(src, dst)
}

/// 把一个目录的内容合并到另一个目录, 子目录不替换
pub fn merge_dir(src: &Path, dst: &Path) -> Result<()> {
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
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
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
    use super::*;
    use path_macro::path;

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
    fn ext_works() {
        let ext = file_ext(&A_JSON);
        assert_eq!(ext, "json");
    }

    #[test]
    fn stem_works() {
        let name = file_name(&A_JSON);
        let stem = file_stem(&A_JSON);
        assert_eq!(name, "a.json");
        assert_eq!(stem, "a");
    }

    #[test]
    fn file_name_append_works() {
        let f1 = PathBuf::from(&A_JSON);
        let f2 = PathBuf::from("/tmp/a_1.json");
        let p = file_name_append(&f1, "_1");
        assert_eq!(f2, p);
    }

    #[test]
    fn make_parent_works() {
        let p = "/etc/passwd";
        assert_eq!(make_parent(&p).is_ok(), true);

        let p = "/etc/passwd/abc";
        assert_eq!(make_parent(&p).is_ok(), false);
    }

    #[test]
    fn link_works() {
        let src = "/home";
        let dst = "/tmp/home";
        relink(src, dst).unwrap();
    }

    use tempfile::tempdir;

    #[test]
    fn test_merge_dir() {
        // 创建临时目录
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src/1");
        let dst_dir = dir.path().join("dst/1");

        // 创建源目录和目标目录
        fs::create_dir_all(&src_dir).unwrap();
        fs::create_dir_all(&dst_dir).unwrap();

        // 在源目录中创建文件
        let mut file = File::create(src_dir.join("test1.txt")).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // 在目标目录中创建文件
        let mut file = File::create(dst_dir.join("test2.txt")).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // 执行合并操作
        merge_dir(&src_dir, &dst_dir).unwrap();

        // 检查目标目录中是否存在源目录中的文件
        assert!(dst_dir.join("test1.txt").exists());
        assert!(dst_dir.join("test2.txt").exists());
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
