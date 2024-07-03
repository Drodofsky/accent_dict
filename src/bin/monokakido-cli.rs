use std::io::Write;

use accent_dict::{Error, MonokakidoDict};

fn print_help() {
    println!("Monokakido CLI. Supported subcommands:");
    println!("list_items {{dict}} {{keyword}} - lists all items");
    println!("list_audio {{dict}} {{keyword}} - lists all audio files");
    println!("get_audio {{dict}} {{id}} - writes an audio file to stdout");
    println!("help - this help");
}

fn list_items(dict_name: &str, keyword: &str) -> Result<(), Error> {
    let mut dict = MonokakidoDict::open()?;
    let (_, items) = dict.keys.search_exact(keyword)?;

    for id in items {
        let item = dict.pages.get_item(id)?;
        println!("{item}");
    }
    Ok(())
}

fn list_pages(dict_name: &str, keyword: &str) -> Result<(), Error> {
    let mut dict = MonokakidoDict::open()?;
    let (_, items) = dict.keys.search_exact(keyword)?;

    for id in items {
        let page = dict.pages.get_page(id)?;
        println!("{page}");
    }
    Ok(())
}

fn list_audio(dict_name: &str, keyword: &str) -> Result<(), Error> {
    let mut dict = MonokakidoDict::open()?;
    let (_, items) = dict.keys.search_exact(keyword)?;

    for id in items {
        for audio in dict.pages.get_item_audio(id)? {
            if let Some((_, audio)) = audio?.split_once("href=\"") {
                if let Some((id, _)) = audio.split_once('"') {
                    println!("{id}");
                }
            }
        }
    }
    Ok(())
}

fn get_audio(dict_name: &str, id: &str) -> Result<(), Error> {
    let id = id.strip_suffix(".aac").unwrap_or(id);
    let mut dict = MonokakidoDict::open()?;
    let aac = dict.audio.get(id)?;
    let mut stdout = std::io::stdout().lock();
    // TODO: for ergonomics/failsafe, check if stdout is a TTY
    stdout.write_all(aac)?;
    Ok(())
}

fn main() {
    let mut args = std::env::args();
    let res = match args.nth(1).as_deref() {
        Some("list_audio") => {
            if let (Some(dict_name), Some(keyword)) = (args.next(), args.next()) {
                list_audio(&dict_name, &keyword)
            } else {
                Err(Error::InvalidArg)
            }
        }
        Some("get_audio") => {
            if let (Some(dict_name), Some(id)) = (args.next(), args.next()) {
                get_audio(&dict_name, &id)
            } else {
                Err(Error::InvalidArg)
            }
        }
        Some("list_items") => {
            if let (Some(dict_name), Some(keyword)) = (args.next(), args.next()) {
                list_items(&dict_name, &keyword)
            } else {
                Err(Error::InvalidArg)
            }
        }
        Some("list_pages") => {
            if let (Some(dict_name), Some(keyword)) = (args.next(), args.next()) {
                list_pages(&dict_name, &keyword)
            } else {
                Err(Error::InvalidArg)
            }
        }
        None | Some("help") => {
            print_help();
            Ok(())
        }
        _ => Err(Error::InvalidSubcommand),
    };

    if let Err(e) = res {
        eprintln!("Error: {e:?}");
        std::process::exit(1)
    }
}
