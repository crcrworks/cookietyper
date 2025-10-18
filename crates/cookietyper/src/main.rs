use std::{io::stdin, thread};

use cookietyper_core::Game;

enum Event {
    EarnCookies(i32),
    ShowCookiesAmount,
    ShowCps,
    InvalidCommand,
}

impl From<String> for Event {
    fn from(s: String) -> Self {
        let s = s.trim();

        if !s.starts_with("/") {
            let s_num = s.len();
            return Event::EarnCookies(s_num as i32);
        }

        match s {
            "/cc" => Event::ShowCookiesAmount,
            "/cps" => Event::ShowCps,
            _ => Event::InvalidCommand,
        }
    }
}

fn main() {
    let mut game = Game::default();

    let (tx, rx) = flume::bounded(5);

    thread::spawn(move || {
        loop {
            let mut s = String::new();

            if let Err(e) = stdin().read_line(&mut s) {
                eprintln!("{e}");
            }

            let _ = tx.send(Event::from(s));
        }
    });

    loop {
        if let Ok(event) = rx.try_recv() {
            match event {
                Event::EarnCookies(n) => {
                    for _ in 0..n {
                        game.earn_cookies();
                    }
                }
                Event::ShowCookiesAmount => {
                    let current_cookies = game.current_cookies();
                    println!("{current_cookies}");
                }
                Event::ShowCps => {
                    let cps = game.cps();
                    println!("{cps}");
                }
                Event::InvalidCommand => {
                    println!("InvalidCommand");
                }
            }
        }

        game.update();
    }
}
