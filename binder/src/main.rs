#[macro_use]
extern crate clap;
extern crate html5ever;
#[macro_use]
extern crate serde_derive;

use std::path::Path;

mod binder;
mod encoding;
mod html;

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

    let root = Path::new("~/repo/binder");

    //let files = fs::read_dir(root);

    //TODO: 命令行解析，数据库

    file!();
}
