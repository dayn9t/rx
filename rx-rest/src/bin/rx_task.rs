use anyhow::anyhow;
use chrono::Local;
use clap::{Parser, Subcommand};
use colored::Colorize;
use prettytable::{Table, row};
use rx_core::time::LocalDateTime;
use rx_rest::task::TaskClient;
use rx_rest::task::{COMPLETED, ERROR, IN_PROGRESS, NOT_STARTED, TaskInfo};
use std::{fs, path::Path, process};

/// 配置类，存储全局配置
#[derive(Debug, Clone)]
struct Config {
    /// 服务器URL
    pub url: String,
    /// 任务表名
    pub task_table: String,
    /// 状态表名
    pub status_table: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: "http://localhost:8080/api".to_string(),
            task_table: "tasks".to_string(),
            status_table: "statuses".to_string(),
        }
    }
}

/// 将日期时间格式化为简洁格式
fn format_datetime(dt: &Option<LocalDateTime>) -> String {
    match dt {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "".to_string(),
    }
}

/// 获取任务状态文本描述
fn get_status_text(status: i32) -> &'static str {
    match status {
        s if s == NOT_STARTED as i32 => "未启动",
        s if s == IN_PROGRESS as i32 => "进行中",
        s if s == COMPLETED as i32 => "已完成",
        s if s == ERROR as i32 => "出错",
        _ => "未知",
    }
}

/// 任务管理命令行工具
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 服务器URL
    #[arg(short, long)]
    url: Option<String>,

    /// 任务表名称
    #[arg(long = "task-table")]
    task_table: Option<String>,

    /// 状态表名称
    #[arg(long = "status-table")]
    status_table: Option<String>,

    /// 子命令
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 列出所有任务
    List,

    /// 列出所有任务状态
    Status,

    /// 添加新任务
    Add {
        /// 任务名称
        #[arg(short, long)]
        name: String,

        /// 任务类型
        #[arg(short, long)]
        r#type: u32,

        /// 任务数据，JSON格式或文件路径
        #[arg(short, long)]
        data: String,

        /// 任务描述
        #[arg(long)]
        desc: Option<String>,
    },

    /// 开始执行指定任务
    Start {
        /// 任务ID
        task_id: String,

        /// 工作者标识
        #[arg(short, long)]
        worker: Option<String>,
    },

    /// 更新任务进度
    Update {
        /// 任务ID
        task_id: String,

        /// 任务进度(0-100)
        #[arg(short, long)]
        progress: u32,

        /// 任务状态码
        #[arg(short, long)]
        status: Option<i32>,
    },

    /// 标记任务为已完成
    Complete {
        /// 任务ID
        task_id: String,
    },

    /// 标记任务为出错状态
    Error {
        /// 任务ID
        task_id: String,
    },

    /// 获取指定任务的详细信息和状态
    Info {
        /// 任务ID
        task_id: String,
    },

    /// 查找下一个可执行的任务
    Next,

    /// 启用或禁用任务
    Enable {
        /// 任务ID
        task_id: String,

        /// 是否启用任务
        #[arg(long, default_value = "true")]
        enable: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 创建配置
    let mut config = Config::default();

    // 更新配置
    if let Some(url) = cli.url {
        config.url = url;
    }
    if let Some(task_table) = cli.task_table {
        config.task_table = task_table;
    }
    if let Some(status_table) = cli.status_table {
        config.status_table = status_table;
    }

    // 创建任务客户端
    let client = TaskClient::new(
        &config.url,
        Some(&config.task_table),
        Some(&config.status_table),
    );

    // 执行对应的子命令
    match cli.command {
        Commands::List => list_tasks(&client).await?,
        Commands::Status => list_statuses(&client).await?,
        Commands::Add {
            name,
            r#type,
            data,
            desc,
        } => add_task(&client, name, r#type, data, desc).await?,
        Commands::Start { task_id, worker } => start_task(&client, &task_id, worker).await?,
        Commands::Update {
            task_id,
            progress,
            status,
        } => update_progress(&client, &task_id, progress, status).await?,
        Commands::Complete { task_id } => complete_task(&client, &task_id).await?,
        Commands::Error { task_id } => mark_error(&client, &task_id).await?,
        Commands::Info { task_id } => get_task_info(&client, &task_id).await?,
        Commands::Next => find_next_task(&client).await?,
        Commands::Enable { task_id, enable } => enable_task(&client, &task_id, enable).await?,
    }

    Ok(())
}

/// 列出所有任务
async fn list_tasks(client: &TaskClient) -> anyhow::Result<()> {
    let result = client.get_all_tasks(None).await;

    if let Err(e) = result {
        eprintln!("{}", format!("获取任务列表失败: {}", e).red());
        process::exit(1);
    }

    let tasks = result.unwrap();
    if tasks.is_empty() {
        println!("{}", "没有找到任何任务".yellow());
        return Ok(());
    }

    // 创建表格
    let mut table = Table::new();
    table.add_row(row!["ID", "名称", "类型", "创建时间", "数据", "描述"]);
    for task in tasks {
        table.add_row(row![
            task.id.unwrap_or_default(),
            task.name.unwrap_or_default(),
            task.r#type.to_string(),
            format_datetime(&task.created_at),
            task.data,
            task.desc.unwrap_or_default(),
        ]);
    }

    table.printstd();
    Ok(())
}

/// 列出所有任务状态
async fn list_statuses(client: &TaskClient) -> anyhow::Result<()> {
    let result = client.get_all_statuses(None).await;

    if let Err(e) = result {
        eprintln!("{}", format!("获取任务状态列表失败: {}", e).red());
        process::exit(1);
    }

    let statuses = result.unwrap();
    if statuses.is_empty() {
        println!("{}", "没有找到任何任务状态".yellow());
        return Ok(());
    }

    // 创建表格
    let mut table = Table::new();
    table.add_row(row![
        "任务ID",
        "状态",
        "进度",
        "开始时间",
        "更新时间",
        "启用"
    ]);
    for status in statuses {
        table.add_row(row![
            status.id.unwrap_or_default(),
            get_status_text(status.status),
            format!("{}%", status.progress),
            format_datetime(&status.start_time),
            format_datetime(&status.update_time),
            if status.enabled { "是" } else { "否" },
        ]);
    }

    table.printstd();
    Ok(())
}

/// 添加新任务
async fn add_task(
    client: &TaskClient,
    name: String,
    task_type: u32,
    data: String,
    desc: Option<String>,
) -> anyhow::Result<()> {
    // 判断data是否为文件路径
    let data_content = if Path::new(&data).is_file() {
        fs::read_to_string(&data).map_err(|e| anyhow!("读取数据文件失败: {}", e))?
    } else {
        data
    };

    // 创建任务对象
    let task = TaskInfo {
        id: None,
        name: Some(name),
        r#type: task_type,
        created_at: Some(Local::now()),
        desc,
        data: data_content,
    };

    // 添加任务
    let result = client.add_task(task).await;

    if let Err(e) = result {
        eprintln!("{}", format!("添加任务失败: {}", e).red());
        process::exit(1);
    }

    let new_task = result.unwrap();
    println!(
        "{}",
        format!("成功添加任务: {}", new_task.id.unwrap_or_default()).green()
    );
    Ok(())
}

/// 开始执行指定任务
async fn start_task(
    client: &TaskClient,
    task_id: &str,
    worker: Option<String>,
) -> anyhow::Result<()> {
    let result = client.task_start(task_id, worker).await;

    if let Err(e) = result {
        eprintln!("{}", format!("启动任务失败: {}", e).red());
        process::exit(1);
    }

    let status = result.unwrap();
    println!(
        "{}",
        format!("成功启动任务: {}", status.id.unwrap_or_default()).green()
    );
    Ok(())
}

/// 更新任务进度
async fn update_progress(
    client: &TaskClient,
    task_id: &str,
    progress: u32,
    status: Option<i32>,
) -> anyhow::Result<()> {
    let result = client.update_progress(task_id, progress, status).await;

    if let Err(e) = result {
        eprintln!("{}", format!("更新任务进度失败: {}", e).red());
        process::exit(1);
    }

    let status = result.unwrap();
    println!(
        "{}",
        format!(
            "成功更新任务进度: {}，当前进度: {}%",
            status.id.unwrap_or_default(),
            status.progress
        )
        .green()
    );
    Ok(())
}

/// 标记任务为已完成
async fn complete_task(client: &TaskClient, task_id: &str) -> anyhow::Result<()> {
    let result = client.task_done(task_id).await;

    if let Err(e) = result {
        eprintln!("{}", format!("完成任务失败: {}", e).red());
        process::exit(1);
    }

    let status = result.unwrap();
    println!(
        "{}",
        format!("成功完成任务: {}", status.id.unwrap_or_default()).green()
    );
    Ok(())
}

/// 标记任务为出错状态
async fn mark_error(client: &TaskClient, task_id: &str) -> anyhow::Result<()> {
    let result = client.task_error(task_id).await;

    if let Err(e) = result {
        eprintln!("{}", format!("标记任务出错失败: {}", e).red());
        process::exit(1);
    }

    let status = result.unwrap();
    println!(
        "{}",
        format!("成功标记任务为出错状态: {}", status.id.unwrap_or_default()).green()
    );
    Ok(())
}

/// 获取指定任务的详细信息和状态
async fn get_task_info(client: &TaskClient, task_id: &str) -> anyhow::Result<()> {
    // 获取任务信息
    let client_lock = client.client.lock().await;
    let task_result = client_lock
        .get::<TaskInfo>(&client.task_table_name, task_id)
        .await;
    drop(client_lock);

    let status_result = client.get_task_status(task_id).await;

    if let Err(e) = task_result {
        eprintln!("{}", format!("获取任务信息失败: {}", e).red());
        process::exit(1);
    }

    if let Err(e) = status_result {
        eprintln!("{}", format!("获取任务状态失败: {}", e).red());
        process::exit(1);
    }

    let task = task_result.unwrap();
    let status = status_result.unwrap();

    // 显示任务详细信息
    println!("{}", "任务信息".blue());
    println!("ID: {}", task.id.unwrap_or_default());
    println!("名称: {}", task.name.unwrap_or_default());
    println!("类型: {}", task.r#type);
    println!("创建时间: {}", format_datetime(&task.created_at));
    println!("描述: {}", task.desc.unwrap_or_else(|| "无".to_string()));
    println!(
        "数据: {}",
        if task.data.len() > 100 {
            format!("{}...", &task.data[0..100])
        } else {
            task.data
        }
    );

    println!("\n{}", "任务状态".blue());
    println!("状态: {}", get_status_text(status.status));
    println!("进度: {}%", status.progress);
    println!(
        "开始时间: {}",
        if status.start_time.is_some() {
            format_datetime(&status.start_time)
        } else {
            "未开始".to_string()
        }
    );
    println!(
        "更新时间: {}",
        if status.update_time.is_some() {
            format_datetime(&status.update_time)
        } else {
            "无".to_string()
        }
    );
    println!(
        "启用状态: {}",
        if status.enabled {
            "已启用"
        } else {
            "已禁用"
        }
    );
    Ok(())
}

/// 查找下一个可执行的任务
async fn find_next_task(client: &TaskClient) -> anyhow::Result<()> {
    let result = client.find_task().await;

    if let Err(e) = result {
        println!("{}", format!("{}", e).yellow());
        return Ok(());
    }

    let (task, _) = result.unwrap();
    println!("{}", "找到可执行任务:".green());
    println!("ID: {}", task.id.unwrap_or_default());
    println!("名称: {}", task.name.unwrap_or_default());
    println!("类型: {}", task.r#type);
    println!("描述: {}", task.desc.unwrap_or_else(|| "无".to_string()));
    println!("创建时间: {}", format_datetime(&task.created_at));
    Ok(())
}

/// 启用或禁用任务
async fn enable_task(client: &TaskClient, task_id: &str, enable: bool) -> anyhow::Result<()> {
    // 获取当前状态
    let mut status = client.get_task_status(task_id).await?;

    // 更新启用状态
    status.enabled = enable;
    status.update_time = Some(Local::now());

    // 提交更新
    let client_lock = client.client.lock().await;
    let result = client_lock.put(&client.status_table_name, status).await;
    drop(client_lock);

    if let Err(e) = result {
        let action = if enable { "启用" } else { "禁用" };
        eprintln!("{}", format!("{}任务失败: {}", action, e).red());
        process::exit(1);
    }

    let status = result.unwrap();
    let action = if enable { "启用" } else { "禁用" };
    println!(
        "{}",
        format!("已{}任务: {}", action, status.id.unwrap_or_default()).green()
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
