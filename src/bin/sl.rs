extern crate sl;
extern crate ctrlc;
extern crate ncurses;
extern crate getopts;

use getopts::Options;
use std::{env,thread,time};

use sl::Train;
use sl::d51::SL;
use sl::c51::C51;
use sl::logo::Logo;

trait Render: Train + Copy {
    fn render(&self, x: i32) {
        let mut len = 0 as i32;
        let y = ncurses::LINES() / 2;
        let body_iter = self.body().iter();
        let wheelset_iter = self.wheelset(x as usize).iter();
        let iter = body_iter.chain(wheelset_iter);
        let (_, hint) = iter.size_hint();
        let height = match hint {
            Some(s) => s,
            None => panic!("this really shouldn't happen"),
        };
        let offset = (height / 2) as i32;
        for (index, line) in iter.rev().enumerate() {
            if line.len() as i32 > len {
                len = line.len() as i32;
            }
            self.render_line((y + offset) - index as i32, x, *line);
        }
        if let Some(tender) = self.tender() {
            let mut new_len = 0 as i32;
            for (index, line) in tender.iter().rev().enumerate() {
                if len + line.len() as i32 > new_len {
                    new_len = len + line.len() as i32;
                }
                self.render_line((y + offset) - index as i32, x + len, *line);
            }
            len = new_len;
        }
        if let Some(wagon) = self.wagon() {
            for _ in 0..self.wagons() {
                let mut new_len = 0 as i32;
                for (index, line) in wagon.iter().rev().enumerate() {
                    if len + line.len() as i32 > new_len {
                        new_len = len + line.len() as i32;
                    }
                    self.render_line((y + offset) - index as i32, x + len, *line);
                }
                len = new_len;
            }
        }
    }

    fn render_line(&self, y: i32, x: i32, line: &str) {
        let paint_len = (ncurses::COLS() - x) as usize;
        if paint_len < line.len() {
            ncurses::mvaddstr(y, x, &line[0..paint_len]);
        } else if x < 0 {
            if -x < line.len() as i32 {
                ncurses::mvaddstr(y, 0, &line[-x as usize..line.len()]);
            }
        } else {
            ncurses::mvaddstr(y, x, line);
        }
    }
}

impl Render for SL {}
impl Render for C51 {}
impl Render for Logo {}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("l", "", "logo");
    opts.optflag("c", "", "C51");
    opts.optflag("a", "", "reserved for future use");
    opts.optflag("f", "", "reserved for future use");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Ignore SIGINT signal from ctrl-c
    ctrlc::set_handler(|| ());
    ncurses::initscr();
    ncurses::nodelay(ncurses::stdscr(), true);
    ncurses::leaveok(ncurses::stdscr(), true);
    ncurses::scrollok(ncurses::stdscr(), false);
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let cols = ncurses::COLS();
    for x in (-85..cols).rev() {
        ncurses::clear();
        if matches.opt_present("l") {
            Logo.render(x)
        } else if matches.opt_present("c") {
            C51.render(x)
        } else {
            SL.render(x)
        };
        ncurses::getch();
        ncurses::refresh();
        thread::sleep(time::Duration::from_millis(20));
    }
    ncurses::endwin();
}
