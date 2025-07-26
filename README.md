# rx

Rust语言基本扩展库。

TODO: cx是原始版本，提取到rx即可删除。

rx      基本功能
rx-db   数据库相关
rx-net  网络相关
rx-rest REST相关
rx-web  WEB相关

## 越来

```bash
sudo apt install libssl-dev
sudo apt install libssl-dev build-essential cmake
ffmpeg
 sudo apt install libavfilter-dev libavdevice-dev
sudo apt install clang

```


```

class StatusInfo(Record):
    """任务状态记录类

    记录任务的状态信息，用于跟踪任务的执行进度和状态
    """

    status: TaskStatus = TaskStatus.NOT_STARTED
    """任务状态码，0-未启动，1-进行，2-完成，3-出错"""
    progress: int = 0
    """任务进度值，范围从起始进度到结束进度"""
    start_time: Datetime | None = None
    """任务开始执行时间"""
    update_time: Datetime | None = None
    """任务数据更新时间"""
    enabled: bool = True
    """任务是否启用"""
    worker: str | None = None
    """执行任务的工作者标识，例如线程名或进程名"""
```