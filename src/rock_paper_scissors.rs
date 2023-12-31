/*
* Crea un programa que calcule quien gana más partidas al piedra,
* papel, tijera, lagarto, spock.
* - El resultado puede ser: "Player 1", "Player 2", "Tie" (empate)
* - La función recibe un listado que contiene pares, representando cada jugada.
* - El par puede contener combinaciones de "🗿" (piedra), "📄" (papel),
*   "✂️" (tijera), "🦎" (lagarto) o "🖖" (spock).
*/

use std::collections::HashMap;

fn rock_paper_scissors(games: Vec<Vec<char>>) -> String {
    let mut rules: HashMap<char, [char; 2]> = HashMap::new();
    rules.insert('r', ['s', 'l']);
    rules.insert('p', ['r', 'o']);
    rules.insert('s', ['p', 'l']);
    rules.insert('l', ['p', 'o']);
    rules.insert('o', ['r', 's']);

    struct Game {
        player1_score: u8,
        player2_score: u8,
    }

    let mut scores = Game {
        player1_score: 0,
        player2_score: 0,
    };

    for game in games {
        let player1_game = game[0];
        let player2_game = game[1];
        if player1_game != player2_game {
            if rules.get(&player1_game).unwrap().contains(&player2_game) {
                scores.player1_score += 1;
            } else {
                scores.player2_score += 1;
            }
        }
    }

    match scores.player1_score {
        x if x > scores.player2_score => "Player1".to_string(),
        x if x < scores.player2_score => "Player2".to_string(),
        _ => "Draw".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_tie() {
        let result = rock_paper_scissors(vec![vec!['r', 'r']]);
        assert_eq!(result, "Draw");
    }

    #[test]
    fn p1_one() {
        let result = rock_paper_scissors(vec![vec!['r', 's']]);
        assert_eq!(result, "Player1");
    }

    #[test]
    fn p1_two() {
        let result = rock_paper_scissors(vec![vec!['s', 'p']]);
        assert_eq!(result, "Player1");
    }

    #[test]
    fn is_tie_two() {
        let result = rock_paper_scissors(vec![
            vec!['r', 'r'],
            vec!['r', 'r'],
            vec!['r', 'r'],
            vec!['r', 'r'],
        ]);
        assert_eq!(result, "Draw");
    }

    #[test]
    fn p1_three() {
        let result = rock_paper_scissors(vec![
            vec!['o', 'r'],
            vec!['s', 'p'],
            vec!['p', 'p'],
            vec!['l', 'o'],
        ]);
        assert_eq!(result, "Player1");
    }
}
