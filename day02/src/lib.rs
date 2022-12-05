pub fn calculate_total_score(s: &str) -> u32 {
    s.lines().flat_map(Round::parse).map(|r| r.score()).sum()
}

pub fn calculate_predicted_score(s: &str) -> u32 {
    s.lines()
        .flat_map(parse_hand_and_outcome)
        .map(|(hand, outcome)| outcome.predict_round(hand).score())
        .sum()
}

fn parse_hand_and_outcome(s: &str) -> Option<(Hand, Outcome)> {
    let mut iter = s.split_ascii_whitespace();

    if let (Some(hand), Some(outcome), None) = (
        iter.next().and_then(Hand::parse),
        iter.next().and_then(Outcome::parse),
        iter.next(),
    ) {
        Some((hand, outcome))
    } else {
        None
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
impl Hand {
    // Table of (X, Y) where "X defeats Y"
    const DEFEAT_TABLE: &'static [(Hand, Hand)] = &[
        (Hand::Rock, Hand::Scissors),
        (Hand::Scissors, Hand::Paper),
        (Hand::Paper, Hand::Rock),
    ];

    fn parse(s: &str) -> Option<Hand> {
        use Hand::*;

        match s {
            "A" | "X" => Some(Rock),
            "B" | "Y" => Some(Paper),
            "C" | "Z" => Some(Scissors),
            _ => None,
        }
    }

    fn score(&self) -> u32 {
        use Hand::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn defeats(&self) -> Hand {
        Hand::DEFEAT_TABLE.iter().find(|e| e.0 == *self).unwrap().1
    }

    fn defeated_by(&self) -> Hand {
        Hand::DEFEAT_TABLE.iter().find(|e| e.1 == *self).unwrap().0
    }
}

#[derive(PartialEq, Debug)]
struct Round {
    opponent_hand_to_play: Hand,
    response_to_play: Hand,
}
impl Round {
    fn parse(s: &str) -> Option<Round> {
        let mut iter = s.split_ascii_whitespace().flat_map(Hand::parse);
        let (Some(opponent_hand_to_play), Some(response_to_play), None) = (iter.next(), iter.next(), iter.next()) else {
            return None;
        };

        Some(Round {
            opponent_hand_to_play,
            response_to_play,
        })
    }

    fn score(&self) -> u32 {
        self.outcome().score() + self.response_to_play.score()
    }

    fn outcome(&self) -> Outcome {
        if self.opponent_hand_to_play == self.response_to_play {
            Outcome::Draw
        } else if self.response_to_play.defeats() == self.opponent_hand_to_play {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}
impl Outcome {
    fn parse(s: &str) -> Option<Self> {
        use Outcome::*;

        match s {
            "X" => Some(Loss),
            "Y" => Some(Draw),
            "Z" => Some(Win),
            _ => None,
        }
    }

    fn score(&self) -> u32 {
        use Outcome::*;

        match self {
            Win => 6,
            Loss => 0,
            Draw => 3,
        }
    }

    fn predict_round(&self, opponent_hand: Hand) -> Round {
        use Outcome::*;

        let response = match self {
            Draw => opponent_hand,
            Loss => opponent_hand.defeats(),
            Win => opponent_hand.defeated_by(),
        };

        Round {
            opponent_hand_to_play: opponent_hand,
            response_to_play: response,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand() {
        assert_eq!(Some(Hand::Rock), Hand::parse("A"));
        assert_eq!(Some(Hand::Paper), Hand::parse("B"));
        assert_eq!(Some(Hand::Scissors), Hand::parse("C"));

        assert_eq!(Some(Hand::Rock), Hand::parse("X"));
        assert_eq!(Some(Hand::Paper), Hand::parse("Y"));
        assert_eq!(Some(Hand::Scissors), Hand::parse("Z"));
    }

    #[test]
    fn test_parse_round() {
        assert_eq!(
            Some(Round {
                opponent_hand_to_play: Hand::Rock,
                response_to_play: Hand::Paper
            }),
            Round::parse("A \t Y")
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            15,
            calculate_total_score(
                "
                A Y
                B X
                C Z
                "
            )
        )
    }

    #[test]
    fn test_round_score_win() {
        assert_eq!(
            8,
            Round {
                opponent_hand_to_play: Hand::Rock,
                response_to_play: Hand::Paper
            }
            .score()
        );
    }

    #[test]
    fn test_round_score_loss() {
        assert_eq!(
            1,
            Round {
                opponent_hand_to_play: Hand::Paper,
                response_to_play: Hand::Rock
            }
            .score()
        );
    }

    #[test]
    fn test_round_score_equal() {
        assert_eq!(
            6,
            Round {
                opponent_hand_to_play: Hand::Scissors,
                response_to_play: Hand::Scissors
            }
            .score()
        );
    }

    #[test]
    fn test_defeats() {
        assert_eq!(Hand::Scissors, Hand::Rock.defeats());
        assert_eq!(Hand::Paper, Hand::Scissors.defeats());
        assert_eq!(Hand::Rock, Hand::Paper.defeats());
    }

    #[test]
    fn test_parse_outcome() {
        assert_eq!(Some(Outcome::Loss), Outcome::parse("X"));
        assert_eq!(Some(Outcome::Draw), Outcome::parse("Y"));
        assert_eq!(Some(Outcome::Win), Outcome::parse("Z"));
    }

    #[test]
    fn test_predicted_score_example() {
        assert_eq!(
            12,
            calculate_predicted_score(
                "
                A Y
                B X
                C Z
                "
            )
        );
    }

    #[test]
    fn test_predict_round_win() {
        assert_eq!(
            Round {
                opponent_hand_to_play: Hand::Rock,
                response_to_play: Hand::Paper
            },
            Outcome::Win.predict_round(Hand::Rock)
        );
    }

    #[test]
    fn test_predict_round_draw() {
        assert_eq!(
            Round {
                opponent_hand_to_play: Hand::Scissors,
                response_to_play: Hand::Scissors
            },
            Outcome::Draw.predict_round(Hand::Scissors)
        );
    }

    #[test]
    fn test_predict_round_loss() {
        assert_eq!(
            Round {
                opponent_hand_to_play: Hand::Paper,
                response_to_play: Hand::Rock
            },
            Outcome::Loss.predict_round(Hand::Paper)
        );
    }
}
