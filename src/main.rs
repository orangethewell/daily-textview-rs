use reqwest; 
use std::io::Read;
use soup::prelude::*;
use chrono::prelude::*;
use directories_next::BaseDirs;
use std::{fs, path::{Path, PathBuf}};
use std::io::prelude::*;
use configparser::ini::Ini;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    author = "Orangethewell",
    version = "1.0",
    about = "Get the daily text by scrapping 'wol.jw.org'.",
    long_about = None
    )]
struct Cli {
    /// Shows the Scripture verse from the text
    #[clap(short='v', long)]
    verse: bool,

    /// Displays the citation from Scripture of the text
    #[clap(short='c', long)]
    citation: bool,

    /// Change the date of the text to display [MODEL: YEAR/MOUNTH/DAY]
    #[clap(short, long)]
    date: Option<String>
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Check if there is any config file
    let path: PathBuf;
    if let Some(base_dirs) = BaseDirs::new() {
        let config_dir = base_dirs.config_dir();
        path = config_dir.join("daily_text-viewer/default.ini");
        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap());
            let mut config_file = fs::File::create(&path)?;
            config_file.write_all(b"[General]\nurl=https://wol.jw.org/pt/wol/h/r5/lp-t/")?; // default as portuguese
        }
    } else {
        panic!("Should not reach there.")
    }
    
    let cli = Cli::parse();
    let mut config = Ini::new();
    config.load(&path);
    let url = config.get("General", "url").unwrap();
    let today: String;
    if cli.date.is_some(){
        today = cli.date.unwrap();
    } else {
        today = Utc::now().date().format("%Y/%m/%d").to_string();
    }
    let mut html = reqwest::blocking::get(format!("{}/{}", url, today))?;
    let mut body = String::new();
    html.read_to_string(&mut body)?;
    let soup = Soup::new(&body);
    
    let verse = soup.tag("p").class("themeScrp").find().expect("Couldn't find verse on text");
    let citation = soup.tag("p").class("sb").find().expect("Couldn't find the citation from the text.");
    if cli.verse {
        println!("{}", verse.text());
    } else if cli.citation {
        println!("{}", citation.text());
    } else {
        println!("{}\n\n{}\n\n{}", Utc::now().date().format("%A, %B %d"), verse.text(), citation.text());
    }
    Ok(())
}
