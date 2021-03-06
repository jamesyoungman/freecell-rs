use freecell::GameState;

// no moves possible
#[test]
fn test_easy_game_over() {
    let game_state = GameState::from_file("test-inputs/easy-game-over.txt").unwrap();
    let actual = game_state.legal_moves();
    assert!(actual.is_empty());
}
