use serde_json::{Value, from_str};

#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action Parse Error!")
    }
}

impl std::error::Error for ParseError {}

pub struct MoveMent {
    pub dx: isize,
    pub dy: isize
}

pub enum ClickBtn {
    Left,
    Right,
}

pub enum Hold {
    Down,
    Up
}

pub enum Wheel {
    Up,
    Down
}

pub enum Action {
    Click(ClickBtn),
    Move(MoveMent),
    Hold(Hold),
    Wheel(Wheel)
}

pub fn prase_data(s: &str) -> Result<Action, ParseError> {
    let value: Result<Value, serde_json::Error> = from_str(s);
    let value = match value {
        Ok(v) => v,
        Err(_) => return Err(ParseError)
    };
    let act = value.get("act").unwrap();
    let act = act.as_str().unwrap();
    // let act = String::from(act);
    let ret = match act {
        _ if act == "click" => {
            let side = value.get("button").unwrap().as_str().unwrap();
            let side = if side == "left" {
                ClickBtn::Left
            } else if side == "right" {
                ClickBtn::Right
            } else {
                ClickBtn::Left
            };
            Ok(Action::Click(side))
        },
        _ if act == "move" => {
            let dx = value.get("dx").unwrap().as_i64().unwrap();
            let dy = value.get("dy").unwrap().as_i64().unwrap();
            let movement = MoveMent {
                dx: dx as isize,
                dy: dy as isize
            };
            Ok(Action::Move(movement))
        },
        _ if act == "hold" => {
            let down = value.get("down").unwrap().as_bool().unwrap();
            if down {
                Ok(Action::Hold(Hold::Down))
            } else {
                Ok(Action::Hold(Hold::Up))
            }
        },
        _ if act == "wheel" => {
            let down = value.get("down").unwrap().as_bool().unwrap();
            if down {
                Ok(Action::Wheel(Wheel::Down))
            } else {
                Ok(Action::Wheel(Wheel::Up))
            }
        },
        _ => Err(ParseError)
    };
    ret
}
