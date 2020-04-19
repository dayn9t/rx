use rx::fs;
use rx_db::*;
use std::collections::HashMap;
use std::path::*;

/// 图书信息
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct BookInfo {
    url: String,
    name: String,
}

/// 章节信息
struct ChapterInfo {
    title: String,
    url: String,
}

/// 目录，存整个文件里
struct CatalogInfo {
    entries: Vec<ChapterInfo>,
}

/// 书架信息
pub struct BookShelf {
    db: DirDb,
    book_tab: DirTable<BookInfo>,
    catalog_tab: DirTable<CatalogInfo>,
    //path: PathBuf,
}

impl BookShelf {
    /// 加载
    pub fn load(path: &Path) -> Result<BookShelf> {
        let db = DirDb::open(&path)?;

        println!("path: {:?}", path);

        let book_tab = DirTable::open(&db, "book")?;
        let catalog_tab = DirTable::open(&db, "calalog")?;

        Ok(BookShelf {
            db,
            book_tab,
            catalog_tab,
        })
    }

    /// 列表
    pub fn list(&self) {
        println!("list all1");
        let rs = self.book_tab.find_pair(0, 0, &|_r| true);
        for (id, r) in rs {
            println!("#{} {} {}", id, r.name, r.url);
        }
    }

    /// 添加
    pub fn add(&mut self, url: &str, name: &Option<&str>) {
        println!("add: {} {:?}", url, name);
        let book = BookInfo {
            url: url.to_string(),
            name: name.unwrap().to_string(),
        };
        self.book_tab.add(book);
    }

    /// 删除
    pub fn remove(&mut self, name: &str) {
        let rs = self.book_tab.find_id(0, 10, &|r| r.name == name);
        if let Some(id) = rs.get(0) {
            self.book_tab.remove(*id);
        }
        println!("remove: {}", name);
    }

    /// 更新
    pub fn update(&mut self, name: &Option<&str>) {
        println!("update: {:?}", name);
        let rs = self.book_tab.find_pair(0, 0, &|_r| true);
        for (id, r) in rs {
            println!("#{} {} {}", id, r.name, r.url);
        }
    }
}
