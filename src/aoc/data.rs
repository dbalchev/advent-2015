use std::{
    error::Error,
    fmt::Display,
    fs::{create_dir_all, File, OpenOptions},
    io::{stdin, BufReader, Read, Write},
    path::PathBuf,
};

use reqwest::blocking::{get, Client};
use scraper::{Html, Selector};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use super::utils::AnyResult;

const YEAR: &str = "2022";

#[derive(Debug)]
struct CannotFindPreError {
    url: String,
}

impl Display for CannotFindPreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot find url {}", self.url)
    }
}

impl Error for CannotFindPreError {}

fn get_file_path(day: i32, last_component: &str) -> PathBuf {
    let mut path = PathBuf::from(".cache");
    path.push(YEAR);
    path.push(format!("{day}"));
    path.push(last_component);
    path
}

fn cached_get(path: PathBuf, actual_get: impl FnOnce() -> AnyResult<String>) -> AnyResult<File> {
    if path.exists() {
        return Ok(OpenOptions::new().read(true).write(false).open(path)?);
    }
    let text = actual_get()?;
    create_dir_all(path.parent().expect("path will always have a parent"))?;
    let mut writable_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path.clone())?;
    writable_file.write_all(text.as_bytes())?;
    drop(writable_file);
    return Ok(OpenOptions::new().read(true).write(false).open(path)?);
}

pub fn get_example_data(day: i32) -> AnyResult<File> {
    cached_get(get_file_path(day, "example.txt"), || {
        let url = format!("https://adventofcode.com/{YEAR}/day/{day}");
        let response = get(url.clone())?;
        let document = Html::parse_document(&response.text()?);

        let pre = document
            .select(&Selector::parse("pre")?)
            .next()
            .ok_or(CannotFindPreError { url })?;
        let text = pre.text().collect::<String>();
        Ok(text)
    })
}

pub fn get_real_data(day: i32) -> AnyResult<File> {
    let mut session_path = PathBuf::from(".cache");
    session_path.push("user_session");
    let user_session_file = cached_get(session_path, || {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(&mut stdout, "Please provide your user session: ")?;
        stdout.flush()?;
        let mut session = String::new();
        stdin().read_line(&mut session)?;
        Ok(session)
    })?;
    let mut user_session = String::new();
    BufReader::new(user_session_file).read_to_string(&mut user_session)?;
    let user_session = user_session.trim();
    cached_get(get_file_path(day, "real.txt"), move || {
        let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input");
        let response = Client::new()
            .get(url)
            .header("cookie", format!("session={user_session};"))
            .send()?;
        Ok(response.text()?)
    })
}
