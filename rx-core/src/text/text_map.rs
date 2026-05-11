use crate::collections::VecMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

/// 文本文件映射
pub struct TextMap {
    vars: VecMap,
}

impl TextMap {
    /// 加载环境变文件
    pub fn load(path: impl AsRef<Path>, sep: char) -> io::Result<Self> {
        let mut vars = VecMap::new();

        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.splitn(2, sep);
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                vars.insert(key, value);
            }
        }

        Ok(Self { vars })
    }

    /// 保存环境变文件
    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        for (key, value) in &self.vars {
            writeln!(file, "{}={}", key, value)?;
        }

        Ok(())
    }

    /// 获取变量值
    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }

    /// 设置变量的值
    pub fn set(&mut self, key: &str, value: &str) {
        self.vars.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_envs() -> io::Result<()> {
        let temp_dir = env::temp_dir();
        let temp_file = temp_dir.join("test_envs.txt");

        // Write some environment variables to the temporary file
        fs::write(&temp_file, "KEY1=VALUE1\nKEY2=VALUE2\n")?;

        // Load the environment variables and check that they were loaded correctly
        let mut envs = TextMap::load(&temp_file, '=')?;
        assert_eq!(envs.get("KEY1"), Some(&"VALUE1".to_string()));
        assert_eq!(envs.get("KEY2"), Some(&"VALUE2".to_string()));

        // Modify the environment variables and save them
        envs.set("KEY1", "NEW_VALUE1");
        envs.set("KEY3", "VALUE3");
        envs.save(&temp_file)?;

        // Reload the environment variables and check that they were saved correctly
        let envs = TextMap::load(&temp_file, '=')?;
        assert_eq!(envs.get("KEY1"), Some(&"NEW_VALUE1".to_string()));
        assert_eq!(envs.get("KEY2"), Some(&"VALUE2".to_string()));
        assert_eq!(envs.get("KEY3"), Some(&"VALUE3".to_string()));

        Ok(())
    }
}
