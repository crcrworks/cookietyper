use std::{io::stdin, thread};

use cookietyper_core::Game;

enum Event {
    EarnCookies(i32),
    ShowCookiesAmount,
    ShowCps,
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

            let s_num = s.trim().len();
            let event = Event::EarnCookies(s_num as i32);

            let _ = tx.send(event);
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
            }
        }

        game.update();
    }
}
