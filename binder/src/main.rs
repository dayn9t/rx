#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use rx::fs;

mod book_shelf;

fn main() {
    let matches = clap_app!(binder =>
        (version: "0.1")
        (author: "J. <dayn9t@gmail.com>")
        (about: "Book binder，bind scattered pages into a book")
        (@subcommand list =>
            (about: "List books")
            (@arg TITLE: "The book title to be ")
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
    )
    .get_matches();

    let root = fs::config_dir_of("binder");
    let mut shelf = book_shelf::BookShelf::load(&root).unwrap();

    if let Some(matches) = matches.subcommand_matches("list") {
        let name = matches.value_of("TITLE");
        shelf.list(&name);
    }
    if let Some(matches) = matches.subcommand_matches("add") {
        let url = matches.value_of("URL").unwrap();
        let title = matches.value_of("TITLE");
        shelf.add(&url, &title);
    }
    if let Some(matches) = matches.subcommand_matches("remove") {
        let id = matches.value_of("ID").unwrap();
        shelf.remove(id);
    }
    if let Some(matches) = matches.subcommand_matches("update") {
        let name = matches.value_of("TITLE");
        shelf.update(&name);
    }
}
