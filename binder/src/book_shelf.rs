use std::fs::File;
use std::io::Write;
use std::path::*;

use rx::{algo, fs};
use rx_db::*;
use rx_web::node::*;

/// 图书信息
#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct BookInfo {
    title: String,
    url: String,
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
    pub fn pull(url: &str) -> Option<CatalogInfo> {
        let root = Node::pull(url)?;
        let title = root.find_title()?;
        let chapters = root.find_max_links();
        Some(CatalogInfo { title, chapters })
    }
}

/// 书架信息
pub struct BookShelf {
    book_tab: DirTable<BookInfo>,
    catalog_tab: DirTable<CatalogInfo>,
    page_dir: PathBuf,
    text_dir: PathBuf,
}

impl BookShelf {
    /// 加载
    pub fn load(path: &Path) -> Result<BookShelf> {
        let db = DirDb::open(&path)?;

        println!("path: {:?}", path);
        Ok(BookShelf {
            book_tab: DirTable::open(&db, &"book")?,
            catalog_tab: DirTable::open(&db, &"calalog")?,
            page_dir: fs::join(&path, &"page"),
            text_dir: fs::join(&path, &"text"),
        })
    }

    /// 列表
    pub fn list(&self, _name: &Option<&str>) {
        println!("list all1");
        let rs = self.book_tab.find_pairs(0, 0, &|_r| true).unwrap();
        for (id, r) in rs {
            println!("#{} {} {}", id, r.title, r.url);
        }
    }

    /// 添加
    pub fn add(&mut self, url: &str, name: &Option<&str>) {
        println!("add: {} {:?}", url, name);
        let book = BookInfo {
            url: url.to_string(),
            title: name.unwrap().to_string(),
        };
        self.book_tab.post(&book).unwrap();
    }

    /// 删除
    pub fn remove(&mut self, name: &str) {
        let rs = self
            .book_tab
            .find_pairs(0, 10, &|r| r.title == name)
            .unwrap();
        if let Some(&(id, _)) = rs.get(0) {
            self.book_tab.delete(id).unwrap();
        }
        println!("remove: {}", name);
    }

    /// 更新
    pub fn update(&mut self, name: &Option<&str>) {
        println!("update: {:?}", name);
        let rs = self.book_tab.find_pairs(0, 0, &|_r| true).unwrap();
        for (id, book) in rs {
            self.update_book(id, book);
        }
    }

    // 更新一本书
    fn update_book(&mut self, book_id: usize, mut book: BookInfo) {
        print!("#{} {} {} ... ", book_id, book.title, book.url);
        if let Some(new) = CatalogInfo::pull(&book.url) {
            if book.title.is_empty() {
                book.title = new.title.clone();
                self.book_tab.put(book_id, &book).unwrap();
            }
            println!("OK");
            let old = self.catalog_tab.get_or_default(book_id);
            let indexes = algo::diff(&new.chapters[..], &old.chapters[..]);
            if !indexes.is_empty() {
                for i in indexes {
                    let chapter = new.chapters.get(i).unwrap();
                    self.save_chapter(chapter, i, book_id);
                }
                self.catalog_tab.put(book_id, &new).unwrap();
                self.bind_pages(&book.title, book_id);
            }
        } else {
            println!("Fail");
        };
    }

    // 拉取/保存正文
    fn save_chapter(&mut self, link: &LinkInfo, chapter_id: usize, book_id: usize) -> Option<()> {
        let root = Node::pull(&link.url)?;
        let text = root.find_max_text();
        let file = self
            .page_dir
            .join(format!("{}/{:04}.txt", book_id, chapter_id));
        let mut file = File::create(file).unwrap();

        let title = format!("\n第{}章 {}\n\n", chapter_id + 1, link.text);
        file.write_all(title.as_bytes()).unwrap();

        for s in text {
            let paragraph = format!("\t{}\n\n", s);
            file.write_all(paragraph.as_bytes()).unwrap();
        }
        Some(())
    }

    // 装订
    fn bind_pages(&self, title: &str, book_id: usize) {
        let page_dir = self.page_dir.join(format!("{}", book_id));
        let book_file = self.text_dir.join(format!("{}.txt", title));
        fs::combine_files_in(&page_dir, &book_file, "txt").unwrap();
    }
}
