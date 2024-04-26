use std::{thread, time};
use std::time::Duration;
use std::io;
use std::io::{stdin, stdout, Read, Write};

use notifica;
use term_size;
use colored::Colorize;


fn main() {
    pomodoro(4, 25., 5.);
}

fn pomodoro(loops: usize, long: f64, short: f64) {
    print!("{}", format!("{loops}").black().bold());
    print!(" times: ");
    print!("{}", format!("{long}").purple());
    print!(" minutes work, ");
    print!("{}", format!("{short}").green()); 
    println!(" minutes rest");
    print!("Press Enter to start ... ");
    for _ in 0..loops {
        pause();
        let work = Progress::new(long);
        let dt = work.dt;
        for x in work {
            print!("{}\r", 
                   format!("{}",x).purple(),
                   );
            io::stdout().flush().unwrap();
            thread::sleep(dt);
        }
        println!("");
        notifica::notify("Take a break!", &format!("Come back in {} minutes", short)).unwrap();

        let stop = Progress::new(short);
        let dt = stop.dt;
        for x in stop {
            print!("{}\r", 
                   format!("{}",x).green(),
                   );
            io::stdout().flush().unwrap();
            thread::sleep(dt);
        }
        notifica::notify("Start working again!", "Press Enter in your terminal").unwrap();
    }
}


fn pause() {
    let mut stdout = stdout();
    stdout.write(b"").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Debug)]
struct Progress {
    dt: Duration,
    width: usize,
    bar: String,
}

impl Progress {
    fn new(minutes: f64) -> Progress {
        let w = match term_size::dimensions() {
            Some((w, _h)) => w,
            _ => panic!("Critical Error"),
        };
        let dt = time::Duration::from_millis((minutes * 60.0e3 / (8.*w as f64)) as u64);
        Progress { 
            dt: dt,
            width: w,
            bar: String::new(),
        }
    }
}

impl Iterator for Progress {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bar.chars().count() >= self.width {
            return None;
        }
        let last = match self.bar.chars().last() {
            Some('\u{2588}') | None => '\u{258F}',
            Some(x) => {
                self.bar.pop();
                char::from_u32(x as u32 - 1).unwrap()
            }
        };
        self.bar.push(last);
        Some(self.bar.clone())
    }
}
