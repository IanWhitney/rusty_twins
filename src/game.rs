extern crate csv;
extern crate rustc_serialize;
use rustc_serialize::Decodable;
use attendee::*;
use std::fmt;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Game {
    pub id: String,
    opponent: String,
    date: String,
    pub attendee_id: String,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}: {}: {}: {}",
               self.id,
               self.opponent,
               self.date,
               Attendee::with_id(&self.attendee_id).unwrap().name)
    }
}

impl Game {
    pub fn attended_by(attendee_id: String) -> Vec<Game> {
        Game::all()
            .into_iter()
            .filter(|g| g.attendee_id == attendee_id)
            .collect::<Vec<Game>>()
    }

    pub fn all() -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        let mut rdr = csv::Reader::from_file("./data/games.csv").unwrap();

        for game in rdr.decode() {
            games.push(game.unwrap());
        }
        games
    }
}
