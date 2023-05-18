mod tennis_match;

use tennis_match::{Player, Player::P1, Player::P2};

fn main() {
    let sequence: Vec<Player> = Vec::from([P1, P1, P2, P2, P1, P2, P1,P1]);
    tennis_match::match_result(sequence);
}
