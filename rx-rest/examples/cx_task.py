from pathlib import Path
from typing import Optional

import typer
from rich import print as rprint
from rich.console import Console
from rich.table import Table

import sys
from jcx.api.task.task_client import TaskClient
from jcx.api.task.task_types import TaskInfo, TaskStatus
from jcx.time.dt_util import now_sh_dt

app = typer.Typer(help="任务管理命令行工具")
console = Console()


def format_datetime(dt) -> str:
    """将日期时间格式化为简洁格式"""
    if dt is None:
        return ""
    return dt.strftime("%Y-%m-%d %H:%M:%S")


class Config:
    """配置类，存储全局配置"""

    # 默认数据库URL
    url: str = "http://localhost:8080/api"
    # 默认表名
    task_table: str = "tasks"
    status_table: str = "statuses"
    # TaskClient实例
    client: Optional[TaskClient] = None


def get_client(base_url: Optional[str] = None) -> TaskClient:
    """获取或创建TaskClient实例"""
    if Config.client is None:
        url = base_url or Config.url
        Config.client = TaskClient(url, Config.task_table, Config.status_table)
    return Config.client


@app.callback()
def callback(
    url: str = typer.Option(None, "--url", "-u", help="服务器URL"),
    task_table: str = typer.Option(None, "--task-table", help="任务表名称"),
    status_table: str = typer.Option(None, "--status-table", help="状态表名称"),
):
    """全局配置选项"""
    if url:
        Config.url = url
    if task_table:
        Config.task_table = task_table
    if status_table:
        Config.status_table = status_table


@app.command("list")
def list_tasks():
    """列出所有任务"""
    client = get_client()
    result = client.get_all_tasks()

    if result.is_err():
        console.print(f"[red]获取任务列表失败: {result.unwrap_err()}[/red]")
        sys.exit(1)

    tasks = result.unwrap()
    if not tasks:
        console.print("[yellow]没有找到任何任务[/yellow]")
        return

    # 创建表格
    table = Table("ID", "名称", "类型", "创建时间", "数据", "描述")
    for task in tasks:
        table.add_row(
            task.id,
            task.name,
            str(task.type),
            format_datetime(task.created_at),
            task.data,
            task.desc or "",
        )

    console.print(table)


@app.command("status")
def list_statuses():
    """列出所有任务状态"""
    client = get_client()
    result = client.get_all_statuses()

    if result.is_err():
        console.print(f"[red]获取任务状态列表失败: {result.unwrap_err()}[/red]")
        sys.exit(1)

    statuses = result.unwrap()
    if not statuses:
        console.print("[yellow]没有找到任何任务状态[/yellow]")
        return

    # 创建表格
    table = Table("任务ID", "状态", "进度", "开始时间", "更新时间", "启用")
    for status in statuses:
        status_text = {
            TaskStatus.PENDING: "未启动",
            TaskStatus.IN_PROGRESS: "进行中",
            TaskStatus.COMPLETED: "已完成",
            TaskStatus.ERROR: "出错",
        }.get(status.status, str(status.status))

        table.add_row(
            status.id,
            status_text,
            f"{status.progress}%",
            format_datetime(status.start_time),
            format_datetime(status.update_time),
            "是" if status.enabled else "否",
        )

    console.print(table)


@app.command("add")
def add_task(
    name: str = typer.Option(..., "--name", "-n", help="任务名称"),
    task_type: int = typer.Option(..., "--type", "-t", help="任务类型"),
    data: str = typer.Option(..., "--data", "-d", help="任务数据，JSON格式或文件路径"),
    desc: Optional[str] = typer.Option(None, "--desc", help="任务描述"),
):
    """添加新任务"""
    # 判断data是否为文件路径
    data_content = data
    if Path(data).is_file():
        try:
            with open(data, "r", encoding="utf-8") as f:
                data_content = f.read()
        except Exception as e:
            console.print(f"[red]读取数据文件失败: {str(e)}[/red]")
            sys.exit(1)

    # 创建任务对象
    task = TaskInfo(
        id="",  # ID会由系统生成
        name=name,
        type=task_type,
        created_at=now_sh_dt(),
        desc=desc,
        data=data_content,
    )

    # 添加任务
    client = get_client()
    result = client.add_task(task)

    if result.is_err():
        console.print(f"[red]添加任务失败: {result.unwrap_err()}[/red]")
        sys.exit(1)

    new_task = result.unwrap()
    console.print(f"[green]成功添加任务[/green]: {new_task.id}")


@app.command("start")
def start_task(
    task_id: str = typer.Argument(..., help="任务ID"),
    worker: Optional[str] = typer.Option(None, "--worker", "-w", help="工作者标识"),
):
    """开始执行指定任务"""
    client = get_client()
    result = client.task_start(task_id, worker)

    if result.is_err():
        console.print(f"[red]启动任务失败: {result.unwrap_err()}[/red]")
        sys.exit(1)

    status = result.unwrap()
    console.print(f"[green]成功启动任务[/green]: {status.id}")


@app.command("update")
def update_progress(
    task_id: str = typer.Argument(..., help="任务ID"),
    progress: int = typer.Option(
        ..., "--progress", "-p", min=0, max=100, help="任务进度(0-100)"
    ),
    status: Optional[int] = typer.Option(None, "--status", "-s", help="任务状态码"),
):
    """更新任务进度"""
    client = get_client()

    task_status = None
    if status is not None:
        try:
            task_status = TaskStatus(status)
        except ValueError:
            console.print(f"[red]无效的状态码: {status}[/red]")
            sys.exit(1)

    result = client.update_progress(task_id, progress, task_status)

    if result.is_err():
        console.print(f"[red]更新任务进度失败: {result.unwrap_err()}[/red]")
        sys.exit(1)

    status = result.unwrap()
    console.print(
        f"[green]成功更新任务进度[/green]: {status.id}，当前进度: {status.progress}%"
    )


@app.command("complete")
def complete_task(task_id: str = typer.Argument(..., help="任务ID")):
    """标记任务为已完成"""
    client = get_client()
    result = client.task_done(task_id)

    if result.is_err():
        console.print(f"[red]完成任务失败: {result.unwrap_err()}[/red]")
        sys.exit(1)

    status = result.unwrap()
    console.print(f"[green]成功完成任务[/green]: {status.id}")


@app.command("error")
def mark_error(task_id: str = typer.Argument(..., help="任务ID")):
    """标记任务为出错状态"""
    client = get_client()
    result = client.task_error(task_id)

    if result.is_err():
        console.print(f"[red]标记任务出错失败: {result.unwrap_err()}[/red]")
        sys.exit(1)

    status = result.unwrap()
    console.print(f"[green]成功标记任务为出错状态[/green]: {status.id}")


@app.command("info")
def get_task_info(task_id: str = typer.Argument(..., help="任务ID")):
    """获取指定任务的详细信息和状态"""
    client = get_client()

    # 获取任务信息
    task_result = client._client.get(TaskInfo, client._task_table_name, task_id)
    status_result = client.get_task_status(task_id)

    if task_result.is_err():
        console.print(f"[red]获取任务信息失败: {task_result.unwrap_err()}[/red]")
        sys.exit(1)

    if status_result.is_err():
        console.print(f"[red]获取任务状态失败: {status_result.unwrap_err()}[/red]")
        sys.exit(1)

    task = task_result.unwrap()
    status = status_result.unwrap()

    # 显示任务详细信息
    console.print("[blue]任务信息[/blue]")
    console.print(f"ID: {task.id}")
    console.print(f"名称: {task.name}")
    console.print(f"类型: {task.type}")
    console.print(f"创建时间: {format_datetime(task.created_at)}")
    console.print(f"描述: {task.desc or '无'}")
    console.print(
        f"数据: {task.data[:100]}..." if len(task.data) > 100 else f"数据: {task.data}"
    )

    console.print("\n[blue]任务状态[/blue]")
    status_text = {
        TaskStatus.PENDING: "未启动",
        TaskStatus.IN_PROGRESS: "进行中",
        TaskStatus.COMPLETED: "已完成",
        TaskStatus.ERROR: "出错",
    }.get(status.status, str(status.status))
    console.print(f"状态: {status_text}")
    console.print(f"进度: {status.progress}%")
    console.print(
        f"开始时间: {status.start_time and format_datetime(status.start_time) or '未开始'}"
    )
    console.print(
        f"更新时间: {status.update_time and format_datetime(status.update_time) or '无'}"
    )
    console.print(f"启用状态: {'已启用' if status.enabled else '已禁用'}")


@app.command("next")
def find_next_task():
    """查找下一个可执行的任务"""
    client = get_client()
    result = client.find_task()

    if result.is_err():
        console.print(f"[yellow]{result.unwrap_err()}[/yellow]")
        return

    task, status = result.unwrap()
    console.print("[green]找到可执行任务[/green]:")
    console.print(f"ID: {task.id}")
    console.print(f"名称: {task.name}")
    console.print(f"类型: {task.type}")
    console.print(f"描述: {task.desc or '无'}")
    console.print(f"创建时间: {format_datetime(task.created_at)}")


@app.command("enable")
def enable_task(
    task_id: str = typer.Argument(..., help="任务ID"),
    enable: bool = typer.Option(True, help="是否启用任务"),
):
    """启用或禁用任务"""
    client = get_client()

    # 获取当前状态
    status_result = client.get_task_status(task_id)
    if status_result.is_err():
        console.print(f"[red]获取任务状态失败: {status_result.unwrap_err()}[/red]")
        sys.exit(1)

    status = status_result.unwrap()
    # 更新启用状态
    status.enabled = enable
    status.update_time = now_sh_dt()

    # 提交更新
    result = client._client.put(client._status_table_name, status)

    if result.is_err():
        console.print(
            f"[red]{'启用' if enable else '禁用'}任务失败: {result.unwrap_err()}[/red]"
        )
        sys.exit(1)

    status = result.unwrap()
    console.print(f"[green]已{'启用' if enable else '禁用'}任务[/green]: {status.id}")


def main():
    """程序入口点"""
    try:
        app()
    except Exception as e:
        rprint(f"[red]程序异常: {str(e)}[/red]")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
