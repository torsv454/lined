use super::super::LineState;

pub fn insert(what: &str, state: &mut LineState) {
    let range = state.insertion_point();
    state.characters.splice(range, what.chars());
    state.cursor += what.len();
}

#[cfg(test)]
mod tests {
    use super::super::super::Cmd;
    use super::super::control::*;
    use super::super::navigation::*;
    use super::*;

    const SAMPLE_NAV: &str = "navigation";

    #[test]
    fn insert_cmd() {
        let mut state = LineState::new(SAMPLE_NAV);
        assert_eq!(0, state.cursor, "|^navigation");

        // insert at start of string
        insert("abc", &mut state);
        assert_eq!("abcnavigation", state.characters.iter().collect::<String>());
        // Cursor is positioned after inserted text.
        assert_eq!(3, state.cursor);

        // insert at end of string
        end(&mut state);
        insert("abc", &mut state);
        assert_eq!("abcnavigationabc", state.text());
        assert_eq!(16, state.cursor);

        // insert in the middle
        repeat(&mut state, 3, &Box::new(Cmd::Back));
        insert("def", &mut state);
        assert_eq!("abcnavigationdefabc", state.text());
        assert_eq!(16, state.cursor);
    }

}
