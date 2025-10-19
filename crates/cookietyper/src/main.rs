use cookietyper_core::Game;
use std::{
    io::stdin,
    thread::{self, sleep},
    time::Duration,
};

mod event;

use crate::event::Event;

const FPS: u64 = 60;

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
                Event::PurchaseFacility => {
                    if let Err(e) = game.purchase_facility() {
                        println!("{e}");
                    }
                }
            }
        }

        game.update();
        sleep(Duration::from_millis((1.0 / FPS as f64 * 1000.0) as u64));
    }
}
