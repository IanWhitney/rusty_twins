extern crate csv;
extern crate rustc_serialize;
extern crate getopts;
mod attendee;
mod game;

use std::fs;
use getopts::Options;
use std::env;
use attendee::*;
use game::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("a", "attendees", "show all attendees");
    opts.optflagopt("g",
                    "games",
                    "show all games, or games for an attendee",
                    "attendee number");
    opts.optopt("p",
                "picker",
                "combined with g to pick a game for an attendee",
                "attendee number");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("a") {
        for attendee in Attendee::all() {
            println!("{}", attendee);
        }
    }

    if matches.opt_present("g") {
        if matches.opt_present("p") {
            let game_id = match matches.opt_str("g") {
                Some(x) => x,
                None => panic!("missing game id"),
            };

            let attendee_id = match matches.opt_str("p") {
                Some(x) => x,
                None => panic!("missing attendee id"),
            };

            let attendee = match Attendee::with_id(&attendee_id) {
                Some(x) => x,
                None => panic!("missing attendee"),
            };

            let games = Game::all();

            let mut wtr = csv::Writer::from_file("./data/ngames.csv").unwrap();

            wtr.write(vec!["id", "opponent", "date", "attendee_id"].into_iter());

            for mut game in games {
                if game.id == game_id {
                    game.attendee_id = attendee.id.clone();
                    println!("{}", game);
                }
                wtr.encode(game).unwrap();
            }
        } else {
            let games = match matches.opt_str("g") {
                Some(x) => Game::attended_by(x),
                None => Game::all(),
            };

            for game in games {
                println!("{}", game);
            }
        }
    }
    fs::copy("./data/ngames.csv", "./data/games.csv");
    fs::remove_file("./data/ngames.csv");
}
