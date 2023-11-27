use std::thread;
use std::time::Duration;

use console::Term;
use console::{style, Style};
use indicatif::ProgressStyle;

use rx_core::fs;
//use rx_net::netplan;

fn net_adaptors() {
    let names = fs::dir_names_in(&"/sys/class/net");
    println!("Hello: {:?}", names);
}

fn main() {
    // /opt/ias/db/abc/tmpl/2eth.yaml

    net_adaptors();

    let term = Term::stdout();
    term.write_line("Hello World!").unwrap();
    thread::sleep(Duration::from_millis(2000));
    term.clear_line().unwrap();
    println!("This is {} neat", style("quite").cyan());
    let cyan = Style::new().cyan();
    println!("This is {} neat", cyan.apply_to("quite"));

    use console::Emoji;
    println!("[3/4] {}Downloading ...", Emoji("🚚 ", ""));
    println!("[4/4] {} Done!", Emoji("✨", ":-)"));

    use indicatif::ProgressBar;

    let bar = ProgressBar::new(1000);

    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    for _ in 0..1000 {
        bar.inc(1);
        // ...
    }
    bar.finish();

    use dialoguer::Confirm;

    if Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    use dialoguer::Input;

    let name = Input::<String>::new()
        .with_prompt("Your name")
        .interact()
        .unwrap();
    println!("Name: {}", name);
    /*
        let mail: String = Input::new()
            .with_prompt("Enter email")
            .validate_with(|input: &str| -> Result<(), &str> {
                if input.contains('@') {
                    Ok(())
                } else {
                    Err("This is not a mail address")
                }
            })
            .interact()
            .unwrap();
    */
}
//ls /sys/class/net
