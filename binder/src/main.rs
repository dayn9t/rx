#[macro_use]
extern crate clap;

use rx_core::fs;

mod book_shelf;

fn main() {
    let matches = clap_app!(binder =>
        (version: "0.2.1 2021-06-05")
        (author: "J. <dayn9t@gmail.com>")
        (about: "Book binder，bind scattered pages into a book")
        (@subcommand list =>
            (about: "List books")
            (@arg TAG: "The book tag to be list")
        )
        (@subcommand add =>
            (about: "Add a new book")
            (@arg URL: +required "URL of pages to be bound")
            (@arg TITLE: "Sets a custom title for the new book")
        )
        (@subcommand remove =>
            (about: "Remove a book by ID")
            (@arg ID: +required "The book to be removed")
        )
        (@subcommand update =>
            (about: "Update book(s)")
            (@arg TITLE: "The book to be updated")
        )
        (@subcommand tag =>
            (about: "Tag a book")
            (@arg ID: +required "The book to be tagged")
            (@arg TAG: +required "The new tag")
        )
        (@subcommand dir =>
            (about: "List directory contents of a book")
            (@arg ID: +required "The book ID")
        )
        (@subcommand bind =>
            (about: "Bind a book")
            (@arg ID: +required "The book to be bound")
            (@arg CHAPTER: "The start chapter ID")
        )
        (@subcommand hosts =>
            (about: "List hosts")
        )
    )
    .get_matches();

    let root = fs::config_dir_of("binder");
    let mut shelf = book_shelf::BookShelf::load(&root).unwrap();

    let r = if let Some(matches) = matches.subcommand_matches("list") {
        let tag = matches.value_of("TAG");
        shelf.list(&tag)
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let url = matches.value_of("URL").unwrap();
        let title = matches.value_of("TITLE");
        shelf.add(&url, &title)
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let id = matches.value_of("ID").unwrap();
        shelf.remove(id)
    } else if let Some(matches) = matches.subcommand_matches("update") {
        let name = matches.value_of("TITLE");
        shelf.update(&name)
    } else if let Some(matches) = matches.subcommand_matches("tag") {
        let id = matches.value_of("ID").unwrap();
        let tag = matches.value_of("TAG").unwrap();
        shelf.tag(id, tag)
    } else if let Some(matches) = matches.subcommand_matches("dir") {
        let id = matches.value_of("ID").unwrap();
        shelf.dir(id)
    } else if let Some(matches) = matches.subcommand_matches("bind") {
        let id = matches.value_of("ID").unwrap();
        let chapter_id = matches.value_of("CHAPTER");
        shelf.bind(id, chapter_id)
    } else if let Some(_matches) = matches.subcommand_matches("hosts") {
        shelf.hosts()
    } else {
        Ok(())
    };

    if let Err(e) = r {
        println!("ERR: {}", e);
    }
}
