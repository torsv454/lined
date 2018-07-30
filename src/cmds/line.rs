use super::super::LineState;

pub fn copy_line(state: &mut LineState) {
    state.clipboard.push(state.characters.clone());
}

pub fn downcase_line(state: &mut LineState) {
    let contents: Vec<char> = state
        .characters
        .drain(..)
        .flat_map(|c| c.to_lowercase())
        .collect();

    state.characters = contents;
}


pub fn upcase_line(state: &mut LineState) {
    let contents: Vec<char> = state
        .characters
        .drain(..)
        .flat_map(|c| c.to_uppercase())
        .collect();

    state.characters = contents;
}

fn first_non_whitespace(state: &LineState) -> Option<usize> {
    state.characters.iter().position(|c| !c.is_whitespace())
}

fn last_non_whitespace(state: &LineState) -> Option<usize> {
    let len = state.characters.len();
    state
        .characters
        .iter()
        .rev()
        .position(|c| !c.is_whitespace())
        .map(|i| len - i)
}

fn trim_to(state: &mut LineState, left: Option<usize>, right: Option<usize>) {
    if let (Some(left), Some(right)) = (left, right) {
        if left != right {
            let _ = state.characters.drain(left..right).last();
        }
    } else {
        state.characters.truncate(0);
    }
}

pub fn trim_line(state: &mut LineState) {
    rtrim_line(state);
    ltrim_line(state);
}
pub fn ltrim_line(state: &mut LineState) {
    let right = first_non_whitespace(state);

    trim_to(state, Some(0), right);
}
pub fn rtrim_line(state: &mut LineState) {
    let left = last_non_whitespace(state);
    let right = Some(state.characters.len());
    trim_to(state, left, right);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn trim_line_cmd() {
        let mut state1 = LineState::new("  hello  ");
        let mut state2 = LineState::new("hejsan  ");
        let mut state3 = LineState::new("  good bye");

        trim_line(&mut state1);
        assert_eq!("hello", state1.text());

        trim_line(&mut state2);
        assert_eq!("hejsan", state2.text());

        trim_line(&mut state3);
        assert_eq!("good bye", state3.text());
    }
}
