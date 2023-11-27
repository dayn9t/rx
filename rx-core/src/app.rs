/// 包信息
#[derive(Debug)]
pub struct PackageInfo {
    /// 名称
    pub name: &'static str,
    /// 版本
    pub version: &'static str,
    /// 作者
    pub authors: &'static str,
    /// 描述信息
    pub description: &'static str,
    /// 构建日期
    pub build_date: &'static str,
}

impl PackageInfo {
    /// 完整版本信息
    pub fn full_version(&self) -> String {
        format!("v{}  build: {}", self.version, self.build_date)
    }
}

/// 创建包信息函数
#[macro_export]
macro_rules! package_function {
    ($func_name:ident) => {
        pub fn $func_name() -> rx_core::app::PackageInfo {
            rx_core::app::PackageInfo {
                name: env!("CARGO_PKG_NAME"),
                version: env!("CARGO_PKG_VERSION"),
                authors: env!("CARGO_PKG_AUTHORS"),
                description: env!("CARGO_PKG_DESCRIPTION"),
                build_date: env!("VERGEN_BUILD_TIMESTAMP"),
                //git_sha: env!("VERGEN_GIT_SHA_SHORT"),
            }
        }
    };
}

/// 应用程序信息(项目包括多个应用程序)
#[derive(Debug)]
pub struct AppInfo {
    /// 应用程序名称
    pub name: String,
    /// 应用程序表述信息
    pub about: String,
    /// 包信息
    pub package: PackageInfo,
}

impl AppInfo {
    /// 创建引用程序信息
    pub fn new(name: &str, about: &str, package: PackageInfo) -> AppInfo {
        AppInfo {
            name: name.to_owned(),
            about: about.to_owned(),
            package,
        }
    }

    /// 完整ID
    pub fn full_name(&self) -> String {
        self.package.name.to_owned() + "-" + &self.name
    }
}
