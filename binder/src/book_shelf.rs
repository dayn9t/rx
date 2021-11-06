use std::collections::HashMap;
use std::{io, thread};
//use std::fs::copy;
use std::fs::File;
use std::io::Write;
use std::path::*;
use std::process::Command;
use std::time::Duration;

use colored::*;
use leg::*;
use http::uri::Uri;

use rx::{algo, fs};
use rx_db::*;
use rx_web::node::*;
use rx_web::req::RequestCfg;

/// 图书信息
#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct BookInfo {
    title: String,
    url: String,
    tag: String,
    chapter_start: usize,
}

impl BookInfo {
    /// 未装订章节起始ID
    pub fn chapter_start(&self) -> usize {
        self.chapter_start.max(1)
    }

    /// 相似
    pub fn like(&self, title: &str) -> bool {
        self.title.find(title).is_some()
    }
}

/// 章节信息
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct ChapterInfo {
    title: String,
    url: String,
}

/// 目录，存整个文件里
#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct CatalogInfo {
    title: String,
    chapters: Vec<LinkInfo>,
}

impl CatalogInfo {
    /// 从URL拉取目录信息
    pub fn pull(url: &str, cfg: &RequestCfg) -> Option<CatalogInfo> {
        let root = Node::pull(url, cfg)?;
        let title = root.find_title()?;
        let mut chapters = root.find_max_links();

        //println!("chapters: {}", serde_json::to_string_pretty(&chapters).unwrap());
        for link in &mut chapters {
            link.complete_by(url);
        }
        Some(CatalogInfo { title, chapters })
    }

    /// 未装订章节结束ID
    pub fn chapter_end(&self) -> usize {
        self.chapters.len() + 1
    }

    /// 未装订章节结束ID
    pub fn total_chapter(&self) -> usize {
        self.chapters.len()
    }
}

/// 存储配置信息
#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StorageCfg {
    path: String,
}

/// 书架配置信息
#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BookShelfCfg {
    request: RequestCfg,
    storage: StorageCfg,
}

/// 书架信息
pub struct BookShelf {
    root: PathBuf,
    book_tab: DirTable<BookInfo>,
    catalog_tab: DirTable<CatalogInfo>,
    page_dir: PathBuf,
    text_dir: PathBuf,
    cfg: BookShelfCfg,
}

/// 命令执行结果
pub type CmdResult = core::result::Result<(), &'static str>;

static INVALID_BOOK: &'static str = "Invalid book ID";
static INVALID_CHAPTER: &'static str = "Invalid chapter ID";
static STORAGE_NOT_FOUND: &'static str = "Storage not found";
static TOO_MANY_STORAGE: &'static str = "Too many storage";
//static FAILED_TO_COPY_THE_BOOK_FILE: &'static str = "Failed to copy the book file";

impl BookShelf {
    /// 加载
    pub fn load(path: &Path) -> Result<BookShelf> {
        let mut db = DirDb::open(&path)?;
        Ok(BookShelf {
            root: path.to_owned(),
            book_tab: db.open_table(&"book")?,
            catalog_tab: db.open_table(&"catalog")?,
            page_dir: path.join("page"),
            text_dir: path.join("text"),
            cfg: db.load_varient("config")?,
        })
    }

    /// 列表
    pub fn list(&self, tag: &Option<&str>) -> CmdResult {
        let tag = tag.unwrap_or("new").to_string();
        //println!("list {:?}", tag);
        let rs = self.book_tab.find_all_pairs().unwrap();
        for (id, book) in rs {
            if book.tag != tag {
                continue;
            }
            let (unbound, total) = if let Ok(catalog) = self.catalog_tab.get(id) {
                (
                    catalog.chapter_end().saturating_sub(book.chapter_start()),
                    catalog.total_chapter(),
                )
            } else {
                (0, 0)
            };
            let title = if unbound > 0 {
                book.title.green()
            } else {
                book.title.normal()
            };
            println!("[{:02}] {}({}/{})\t{}", id, title, unbound, total, book.url);
        }
        Ok(())
    }

    /// 添加
    pub fn add(&mut self, url: &str, name: &Option<&str>) -> CmdResult {
        println!("add: {} {:?}", url, name);
        self.git_pull()?;
        let book = BookInfo {
            url: url.to_string(),
            tag: "new".to_string(),
            title: name.unwrap_or("").to_string(),
            chapter_start: 1,
        };
        self.book_tab.post(&book).unwrap();
        self.git_push()
    }

    /// 删除
    pub fn remove(&mut self, id: &str) -> CmdResult {
        if let Ok(id) = id.parse() {
            self.git_pull()?;
            if let Ok(book) = self.book_tab.get(id) {
                println!("remove: #{} {}", id, book.title);
                self.book_tab.delete(id).unwrap();
                return self.git_push();
            }
        }
        Err(INVALID_BOOK)
    }

    /// 更新
    pub fn update(&mut self, title: &Option<&str>) -> CmdResult {
        let books = if let Some(title) = title {
            self.book_tab
                .find_pairs(0, usize::max_value(), &|r| r.like(title))
        } else {
            self.book_tab.find_all_pairs()
        };
        for (id, book) in books.unwrap() {
            self.update_book(id, book);
        }
        Ok(())
    }

    /// 更新
    pub fn tag(&mut self, book_id: &str, new_tag: &str) -> CmdResult {
        if let Ok(book_id) = book_id.parse() {
            self.git_pull()?;
            if let Ok(mut book) = self.book_tab.get(book_id) {
                println!("tagging book: {}. {}", book_id, book.title);
                book.tag = new_tag.to_string();
                self.book_tab.put(book_id, &book).unwrap();

                return self.git_push();
            }
        }
        Err(INVALID_BOOK)
    }

    // 更新一本书
    fn update_book(&mut self, book_id: usize, mut book: BookInfo) {
        print!("[{:02}] {}\t{} ... ", book_id, book.title, book.url);
        if let Some(new) = CatalogInfo::pull(&book.url, &self.cfg.request) {
            if book.title.is_empty() {
                book.title = new.title.clone();
                self.book_tab.put(book_id, &book).unwrap();
            }

            let mut old = self.catalog_tab.get_or_default(book_id);

            println!("OK");

            let diff = new.chapters.len() as i64 - old.chapters.len() as i64;
            if diff < 0 {
                let msg = format!("The number of chapters has been reduced by {}", -diff);
                warn(&msg, None, None);
            }


            // 检查并且标记那些缺失的章节
            for (i, link) in old.chapters.iter_mut().enumerate() {
                let chapter_id = i + 1;
                let file = self.chapter_file(book_id, chapter_id);
                if !file.exists() {
                    link.text.clear();
                }
            }

            let indexes = algo::diff(&new.chapters[..], &old.chapters[..]);
            if !indexes.is_empty() {
                for i in indexes {
                    let chapter = new.chapters.get(i).unwrap();
                    self.save_chapter(chapter, i + 1, book_id);
                }
                self.catalog_tab.put(book_id, &new).unwrap();
            }
        } else {
            println!("Fail");
        };
    }

    // 拉取/保存正文
    fn save_chapter(&mut self, link: &LinkInfo, chapter_id: usize, book_id: usize) -> Option<()> {
        print!("    {}. {} ...          ", chapter_id, link.text);
        //print!("    {}. {} {}...          ", chapter_id, link.text, link.url);
        if let Some(url) = link.url.as_ref() {
            let root = Node::pull(url, &self.cfg.request)?;
            let text = root.find_max_text();

            let file = self.chapter_file(book_id, chapter_id);
            fs::make_parent(&file).ok()?;
            let mut file = File::create(file).unwrap();

            let title = format!("第{}章 {}\n\n", chapter_id, link.text);
            file.write_all(title.as_bytes()).unwrap();

            for s in text {
                let paragraph = format!("\t{}\n\n", s);
                file.write_all(paragraph.as_bytes()).unwrap();
            }
            println!("OK");
            thread::sleep(Duration::from_secs(1));
            Some(())
        } else {
            println!("Err");
            None
        }
    }

    /// 列出目录
    pub fn dir(&self, book_id: &str) -> CmdResult {
        if let Ok(id) = book_id.parse::<usize>() {
            if let Ok(book) = self.catalog_tab.get(id) {
                for (index, link) in book.chapters.iter().enumerate() {
                    print_chapter(index + 1, &link.text);
                }
                return Ok(());
            }
        }
        Err(INVALID_BOOK)
    }

    /// 装订
    pub fn bind(&mut self, book_id: &str, chapter_id: Option<&str>) -> CmdResult {
        if let Ok(book_id) = book_id.parse() {
            self.git_pull()?;
            if let Ok(mut book) = self.book_tab.get(book_id) {
                println!("binding book: {}. {}", book_id, book.title);
                let catalog = self.catalog_tab.get(book_id).unwrap();

                let chapter_start = if let Some(s) = chapter_id {
                    s.parse::<usize>().map_err(|_e| INVALID_CHAPTER)?.max(1)
                } else {
                    book.chapter_start()
                };

                let mut files = Vec::new();
                for id in chapter_start..catalog.chapter_end() {
                    let title = &catalog.chapters.get(id - 1).unwrap().text;
                    print_chapter(id, &title);
                    files.push(self.chapter_file(book_id, id))
                }
                if files.is_empty() {
                    return Ok(());
                }
                let book_file = self.book_file(&book.title);
                fs::combine_files(&files, &book_file).unwrap();
                self.copy_file(&book_file)?;

                book.chapter_start = catalog.chapter_end();
                self.book_tab.put(book_id, &book).unwrap();
                return self.git_push();
            }
        }
        Err(INVALID_BOOK)
    }

    // 复制装订文件
    fn copy_file(&self, book_file: &Path) -> CmdResult {
        let dirs = fs::mtp_dirs().unwrap();
        //TODO: find_first_dir 查找图书目录
        match dirs.len() {
            0 => Err(STORAGE_NOT_FOUND),
            1 => {
                let dst = dirs
                    .get(0)
                    .unwrap()
                    .join(&self.cfg.storage.path)
                    .join(book_file.file_name().unwrap());

                //TODO: fs::copy往手机上复制失败
                //copy(book_file, &dst).unwrap();
                let output = Command::new("cp").arg(book_file).arg(dst).output().unwrap();
                println!("status: {}", output.status);
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();

                Ok(())
            }
            _ => Err(TOO_MANY_STORAGE),
        }
    }

    // 获取章节文件
    fn chapter_file(&self, book_id: usize, chapter_id: usize) -> PathBuf {
        self.page_dir
            .join(format!("{}/{:04}.txt", book_id, chapter_id))
    }

    // 获取书文件
    fn book_file(&self, title: &str) -> PathBuf {
        self.text_dir.join(format!("{}.txt", title))
    }

    /// 主机列表
    pub fn hosts(&self) -> CmdResult {
        let books = self.book_tab.find_all().unwrap();
        let mut map = HashMap::new();
        for book in &books {
            let uri: Uri = book.url.parse().unwrap();
            let host = uri.host().unwrap().to_string();
            let counter = map.entry(host).or_insert(0);
            *counter += 1;
        }
        let mut vec: Vec<_> = map.iter().collect();
        vec.sort_by_key(|(_k, v)| -*v);

        for (i, (host, counter)) in vec.iter().enumerate() {
            println!("[{:02}] {} ({})", i + 1, host, counter);
        }
        Ok(())
    }

    // 拉取数据
    fn git_pull(&self) -> CmdResult {
        let output = Command::new("git")
            .arg("-C")
            .arg(self.root.clone())
            .arg("pull")
            .output()
            .unwrap();
        println!("pull status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        Ok(())
    }

    // 提交数据
    fn git_push(&self) -> CmdResult {
        let output = Command::new("gac1.sh")
            .arg(self.root.clone())
            .output()
            .unwrap();
        println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        Ok(())
    }
}

// 打印章节条目
fn print_chapter(id: usize, title: &str) {
    println!("    {}. {}", id, title);
}
