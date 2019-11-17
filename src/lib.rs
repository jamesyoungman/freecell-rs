// TODO there is some way to get rid of the extern crate declarations in rust 2018 edition
extern crate arrayvec;
extern crate priority_queue; // TODO: see https://github.com/garro95/priority-queue for tips on speeding it up


pub mod game_state_parser;

mod freecell;
mod state_graph;

#[cfg(test)]
mod tests;



use state_graph::StateGraph;
use freecell::{GameState, Move};



pub fn solve(initial_state: GameState) -> Option<Vec<Move>> {
    let mut state_graph = StateGraph::new(initial_state);
    state_graph.dijkstra()
}
