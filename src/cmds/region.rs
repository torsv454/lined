use super::super::LineState;
use std::cmp;

#[derive(Debug)]
pub struct Region {
    pub start: usize,
    pub end: usize,
}

pub fn region(state: &LineState) -> Region {
    let mark = state.mark.unwrap_or(state.cursor);
    let start = cmp::min(mark, state.cursor);
    let end = cmp::max(mark, state.cursor);

    Region { start, end }
}

pub fn upcase(state: &mut LineState) {
    let Region { start, end } = region(state);

    upcase_sequence(state, start, end);
}

pub fn upcase_sequence(state: &mut LineState, start: usize, end: usize) {
    let contents: Vec<char> = state.characters[start..end]
        .iter()
        .flat_map(|c| c.to_uppercase())
        .collect();

    state.characters.splice(start..end, contents);
}

pub fn downcase(state: &mut LineState) {
    let Region { start, end } = region(state);

    downcase_sequence(state, start, end);
}

pub fn downcase_sequence(state: &mut LineState, start: usize, end: usize) {
    let contents: Vec<char> = state.characters[start..end]
        .iter()
        .flat_map(|c| c.to_lowercase())
        .collect();

    state.characters.splice(start..end, contents);
}

pub fn transpose(state: &mut LineState) {
    let chars_to_move: String = state.characters.drain(0..state.cursor + 1).collect();
    state
        .characters
        .extend(chars_to_move[chars_to_move.len() - 1..].chars());
    state
        .characters
        .extend(chars_to_move[0..chars_to_move.len() - 1].chars());
}
