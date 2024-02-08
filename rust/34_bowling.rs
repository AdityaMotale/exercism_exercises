#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    scores: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { scores: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_game_over() {
            return Err(Error::GameComplete);
        }

        if self.scores.len() % 2 == 1 && self.scores.len() < 18 {
            if self.scores[self.scores.len() - 1] + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }
        if self.scores.len() == 20 && self.scores[18] == 10 && self.scores[19] != 10 {
            if self.scores[19] + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        if pins == 10 {
            if self.scores.len() % 2 == 0 {
                self.scores.push(10);
                if self.scores.len() < 18 {
                    self.scores.push(0);
                }
                return Ok(());
            }
        }

        self.scores.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_over() {
            return None;
        }

        let mut sum = 0;

        for (i, scores) in self.scores.chunks(2).enumerate() {
            if i >= 9 {
                sum += scores.iter().sum::<u16>();
                continue;
            }
            let mut s = scores[0] + scores[1];
            if s == 10 {
                if scores[0] == 10 {
                    let n1 = self.scores[(i + 1) * 2];
                    if n1 == 10 {
                        if i == 8 {
                            s += n1 + self.scores[(i + 1) * 2 + 1];
                        } else {
                            s += n1 + self.scores[(i + 2) * 2];
                        }
                    } else {
                        s += n1 + self.scores[(i + 1) * 2 + 1];
                    }
                } else {
                    s += self.scores[(i + 1) * 2];
                }
            }
            sum += s;
        }

        Some(sum)
    }

    fn is_game_over(&self) -> bool {
        if self.scores.len() == 21 {
            return true;
        } else if self.scores.len() == 20 {
            if self.scores[18] == 10 || (self.scores[18] + self.scores[19] == 10) {
                return false;
            } else {
                return true;
            }
        }

        false
    }
}
