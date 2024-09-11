use std::{
    fs::File,
    io::{stdin, stdout, Write},
    mem::offset_of,
    path::Path,
};
use termion::{cursor, event::Key, input::TermRead, raw::IntoRawMode};
mod ppm;

type uint = u64;

struct Cursor
{
    x: isize,
    y: isize,
}

struct ImageData([i64; 32 * 32]);

struct App
{
    image_data: ImageData,
}
impl App
{
    fn save_file(
        &self,
        name: &str,
    )
    {
    }
}

fn main()
{
    let stdin = stdin();
    let c = Cursor {
        x: 1, y: 1
    };
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        r#"{}{}q to exit, h to print help"#,
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys()
    {
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
        match c.unwrap()
        {
            Key::Char('h') => println!("Hello world!"),
            Key::Char('q') => break,
            Key::Char('x') => (),
            Key::Alt('t') => println!("termion is cool"),
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
