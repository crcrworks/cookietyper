use std::{io::stdin, thread};

use cookietyper_core::Game;

fn main() {
    let mut game = Game::default();

    let (tx, rx) = flume::bounded(5);

    thread::spawn(move || {
        loop {
            let mut s = String::new();

            if let Err(e) = stdin().read_line(&mut s) {
                eprintln!("{e}");
            }

            let _ = tx.send(s.trim().to_string());
        }
    });

    loop {
        if let Ok(s) = rx.try_recv() {
            for _ in 0..s.len() {
                game.earn_cookies();
            }

            let current_cookies = game.current_cookies();
            println!("{current_cookies}");
        }

        game.update();
    }
}
