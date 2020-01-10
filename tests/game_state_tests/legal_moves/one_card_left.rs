use freecell::game_state_parser;
use freecell::Suit::Club;
use freecell::{Card, Move, Position, KING};

use super::utils;

#[test]
fn test_one_card_left() {
    let game_state = game_state_parser::parse_file("test-inputs/one-card-left.txt").unwrap();
    let actual = game_state.legal_moves();
    let expected = vec![
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(0),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(1),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade3.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(3),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade4.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(4),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade5.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(5),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade6.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(6),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade7.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(7),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Foundations,
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecell0.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecell(0),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecell1.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecell(1),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecell2.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecell(2),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecell3.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecell(3),
            },
        ),
    ];

    utils::assert_eq_ignore_order(actual, expected);
}
