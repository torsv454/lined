use super::super::LineState;

pub fn find(what: char, state: &mut LineState) -> bool {
    for i in state.cursor..=state.characters.len() {
        if state.characters[i] == what {
            state.cursor = i;
            return true;
        }
    }
    false
}

pub fn rfind(what: char, state: &mut LineState) -> bool {
    for i in (0..state.cursor).rev() {
        if state.characters[i] == what {
            state.cursor = i;
            return true;
        }
    }
    false
}
