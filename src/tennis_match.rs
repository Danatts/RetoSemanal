/*
* Escribe un programa que muestre cómo transcurre un juego de tenis y quién lo ha ganado.
* El programa recibirá una secuencia formada por "P1" (Player 1) o "P2" (Player 2), según quien
* gane cada punto del juego.
*/

use std::collections::HashMap;

pub enum Player {
    P1,
    P2,
}

struct Score {
    p1: u8,
    p2: u8,
}

pub fn match_result(points: Vec<Player>) {
    let score_table: HashMap<u8, &str> =
        HashMap::from([(0, "Love"), (1, "15"), (2, "30"), (3, "40")]);

    let mut game_score = Score { p1: 0, p2: 0 };

    let mut finished = false;

    println!(
        "{} - {}",
        score_table.get(&game_score.p1).unwrap(),
        score_table.get(&game_score.p2).unwrap()
    );

    for i in points.iter() {
        match i {
            Player::P1 => game_score.p1 += 1,
            Player::P2 => game_score.p2 += 1,
        }
        if game_score.p1 >= 3 && game_score.p2 >= 3 {
            if !finished && (game_score.p1 - game_score.p2).abs_diff(0) <= 1 {
                if game_score.p1 == game_score.p2 {
                    println!("Deuce");
                } else {
                    if game_score.p1 > game_score.p2 {
                        println!("Advance P1");
                    } else {
                        println!("Advance P2");
                    }
                }
            } else {
                finished = true
            }
        } else if game_score.p1 < 4 && game_score.p2 < 4 {
            println!(
                "{} - {}",
                score_table.get(&game_score.p1).unwrap(),
                score_table.get(&game_score.p2).unwrap()
            );
        } else {
            finished = true
        }
    }

    if !finished {
        println!("The game is unfinished");
    } else {
        if game_score.p1 > game_score.p2 {
            println!("P1 Wins")
        } else {
            println!("P2 Wins")
        }
    }
}
