#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

use rx::fs;

mod binder;
mod book_shelf;

fn main() {
    let matches = clap_app!(binder =>
        (version: "0.1")
        (author: "J. <dayn9t@gmail.com>")
        (about: "Book binder，bind scattered pages into a book")
        (@subcommand list =>
            (about: "List all books")
        )
        (@subcommand add =>
            (about: "Add a new book")
            (@arg URL: +required "URL of pages to be bound")
            (@arg BOOK: "Sets a custom name for the new book")
        )
        (@subcommand remove =>
            (about: "Remove a book")
            (@arg BOOK: +required "The book to be removed")
        )
        (@subcommand update =>
            (about: "Update book(s)")
            (@arg BOOK: "The book to be updated")
        )
    )
    .get_matches();

    let root = fs::config_dir_of("binder");
    let mut shelf = book_shelf::BookShelf::load(&root).unwrap();

    if let Some(matches) = matches.subcommand_matches("list") {
        shelf.list();
    }
    if let Some(matches) = matches.subcommand_matches("add") {
        let url = matches.value_of("URL").unwrap();
        let name = matches.value_of("BOOK");
        shelf.add(&url, &name);
    }
    if let Some(matches) = matches.subcommand_matches("remove") {
        let name = matches.value_of("BOOK").unwrap();
        shelf.remove(&name);
    }
    if let Some(matches) = matches.subcommand_matches("update") {
        let name = matches.value_of("BOOK");
        shelf.update(&name);
    }
}
