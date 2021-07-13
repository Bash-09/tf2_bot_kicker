use core::fmt;


#[derive(PartialEq, Eq)]
pub enum Team {
    RED,
    BLU,
    NONE,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: &str;
        match self {
            Team::RED => {out = "RED"},
            Team::BLU => {out = "BLU"},
            Team::NONE => {out = "NONE"},
        }
        write!(f, "{}", out)
    }
}

#[derive(PartialEq, Eq)]
pub enum State {
    Spawning,
    Active,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: &str;
        match self {
            State::Active => {out = "Active"},
            State::Spawning => {out = "Spawning"},
        }
        write!(f, "{}", out)
    }
}


pub struct Player {

    pub userid: String,
    pub name: String,
    pub uniqueid: String, 
    pub time: u32,
    pub team: Team,
    pub state: State,

}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}, \tUID: {}, UUID: {}, Time: {}, State: {}", self.team, self.name, self.userid, self.uniqueid, self.time, self.state)
    }
}