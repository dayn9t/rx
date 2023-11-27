#[macro_use]
extern crate clap;

use rx_core::fs;

fn main() {
    let matches = clap_app!(rxfst =>
        (version: "0.2")
        (author: "J. <dayn9t@gmail.com>")
        (about: "RX File System Toolkit，a collection of file system tools")
        (@subcommand hash =>
            (about: "计算目录内所有文件的HASH")
            (@arg DIR: +required "待计算HASH的目录")
        )
        (@subcommand deldup =>
            (about: "删除与参照文件中HASH重复的文件")
            (@arg DIR: +required "待删除文件所在目录")
            (@arg HASH_FILE: +required "参照HASH文件")
        )
        (@subcommand hosts =>
            (about: "List hosts")
        )
    )
    .get_matches();

    let _root = fs::config_dir_of("rx-fst");
    //let mut shelf = book_shelf::BookShelf::load(&root).unwrap();

    let _r = if let Some(matches) = matches.subcommand_matches("hash") {
        let _dir = matches.value_of("DIR").unwrap();
        //shelf.add(&url, &title)
    } else if let Some(matches) = matches.subcommand_matches("deldup") {
        let _dir = matches.value_of("DIR").unwrap();
        let _hash_file = matches.value_of("HASH_FILE").unwrap();
        //shelf.tag(id, tag)
    } else {
        //Ok(())
    };

    //if let Err(e) = r {
    //        println!("ERR: {}", e);
    //  }
}
