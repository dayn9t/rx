//! 用于测试程序的模块
//!
//! 该模块提供了一系列用于测试的实用工具和宏，
//! 帮助开发者更方便地编写和运行测试代码。

/// 构建相对于当前文件所在目录的路径
///
/// # 示例
///
/// ```ignore
/// let data_path = cur_path!("data.txt");
/// let config_path = cur_path!("config/settings.toml");
/// ```
#[macro_export]
macro_rules! cur_path {
    ($file:expr) => {
        ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(::std::path::Path::new(file!()).parent().unwrap())
            .join($file)
    };
}

/// 构建项目根目录下 assets 文件夹中的路径
///
/// # 示例
///
/// ```ignore
/// let image_path = assets_path!("images/logo.png");
/// let config_path = assets_path!("config.json");
/// ```
#[macro_export]
macro_rules! assets_path {
    ($file:expr) => {
        ::std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join($file)
    };
}
