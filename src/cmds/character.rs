//! Character level functions.
//!
//!
use super::super::LineState;
use std::cmp;
use std::collections::HashMap;

/// Transposes the character at the cursor position with the preceeding character and moves forward.
/// Does nothing if the cursor is in the HOME position.
pub fn transpose_character(state: &mut LineState) {
    if state.characters.len() > 1 {
        if state.cursor == 0 {
            super::navigation::forward(state);
        } else if state.at_end() {
            super::navigation::back(state);
        }
        let first = cmp::max(0, state.cursor - 1);
        let second = cmp::min(state.cursor, state.characters.len() - 1);

        if first != second {
            state.characters.swap(first, second);
            super::navigation::forward(state);
        }
    }
}

/// Upcases the character at the cursor position, Does nothing if the cursor is in the END position.
pub fn upcase_character(state: &mut LineState) {
    if state.at_character() {
        let upcased = state.characters[state.cursor].to_uppercase();
        let target = state.at_cursor();
        state.characters.splice(target, upcased);
    }
}

/// Downcases the character at the cursor position. Does nothing if the cursor is in the END position.  
pub fn downcase_character(state: &mut LineState) {
    if state.at_character() {
        let downcased = state.characters[state.cursor].to_lowercase();
        let target = state.at_cursor();
        state.characters.splice(target, downcased);
    }
}

/// Translates the character at the cursor position. Does nothing if the cursor is in the END position.
pub fn translate_char(state: &mut LineState, table: &HashMap<char, char>) {
    if state.at_character() {
        if let Some(replacement) = table.get(&state.characters[state.cursor]) {
            state.characters[state.cursor] = *replacement;
        }
    }
}

/// Delete the character at the cursor position. Does nothing if the cursor is in the END position.
pub fn delete(state: &mut LineState) {
    if state.at_character() {
        state.characters.remove(state.cursor);
    }
}

/// Delete the character before cursor position. Does nothing if the cursor is in the HOME position.
pub fn delete_before(state: &mut LineState) {
    if state.cursor > 0 {
        state.characters.remove(state.cursor - 1);
        state.cursor -= 1;
    }
}

pub fn translate(state: &mut LineState, table: &HashMap<char, char>) {
    state.characters.iter_mut().for_each(|c| {
        if let Some(&t) = table.get(c) {
            *c = t;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::super::clipboard::*;
    use super::super::navigation::*;
    use super::*;

    const SAMPLE: &str = "navigation";
    const SAMPLE_UPCASED: &str = "NAVIGATION";
    const NAV_LAST: usize = 10;
    const NAV_LAST_CHARP: usize = 9;

    fn copy_all(state: &mut LineState) {
        home(state);
        mark(state);
        end(state);
        copy(state);
    }

    #[test]
    fn delete_cmd() {
        let mut state = LineState::new(SAMPLE);
        assert_eq!(0, state.cursor, "|^navigation");

        delete(&mut state);
        assert_eq!("avigation", state.text());
        assert_eq!(0, state.cursor);

        delete(&mut state);
        assert_eq!("vigation", state.text());
        assert_eq!(0, state.cursor);

        // end of line, can't delete that.
        end(&mut state);
        delete(&mut state);
        assert_eq!("vigation", state.text());
        assert_eq!(8, state.cursor);
    }

    #[test]
    fn delete_before_cmd() {
        let mut state = LineState::new(SAMPLE);
        assert_eq!(0, state.cursor, "|^navigation");

        delete_before(&mut state);
        assert_eq!("navigation", state.text());
        assert_eq!(0, state.cursor);

        forward(&mut state);
        delete_before(&mut state);
        assert_eq!("avigation", state.text());
        assert_eq!(0, state.cursor);

        end(&mut state);
        delete_before(&mut state);
        assert_eq!("avigatio", state.text());
        assert_eq!(8, state.cursor);
    }

    #[test]
    fn upcase_character_cmd() {
        let mut state = LineState::new(SAMPLE);

        upcase_character(&mut state);
        assert_eq!("Navigation", state.text());
        assert_eq!(0, state.cursor);

        // Do nothing at END position
        end(&mut state);
        upcase_character(&mut state);
        assert_eq!("Navigation", state.text());
        assert_eq!(NAV_LAST, state.cursor);

        // Change last character when in LAST position.
        last(&mut state);
        upcase_character(&mut state);
        assert_eq!("NavigatioN", state.text());
        assert_eq!(NAV_LAST_CHARP, state.cursor);

        // Is idempotent
        upcase_character(&mut state);
        assert_eq!("NavigatioN", state.text());
        assert_eq!(NAV_LAST_CHARP, state.cursor);
    }

    #[test]
    fn downcase_character_cmd() {
        let mut state = LineState::new(SAMPLE_UPCASED);

        downcase_character(&mut state);
        assert_eq!("nAVIGATION", state.text());
        assert_eq!(0, state.cursor);

        // Do nothing at END position
        end(&mut state);
        upcase_character(&mut state);
        assert_eq!("nAVIGATION", state.text());
        assert_eq!(NAV_LAST, state.cursor);

        last(&mut state);
        downcase_character(&mut state);
        assert_eq!("nAVIGATIOn", state.text());
        assert_eq!(NAV_LAST_CHARP, state.cursor);

        // Is idempotent
        downcase_character(&mut state);
        assert_eq!("nAVIGATIOn", state.text());
        assert_eq!(NAV_LAST_CHARP, state.cursor);
    }

    #[test]
    fn translate_character_cmd() {
        let mut state = LineState::new(SAMPLE);

        let mut table = HashMap::new();
        table.insert('n', 'M');
        table.insert('N', 'M');
        table.insert('M', 'N');
        table.insert('a', 'o');

        translate_char(&mut state, &table);
        assert_eq!("Mavigation", state.text());
        assert_eq!(0, state.cursor);

        forward(&mut state);
        translate_char(&mut state, &table);
        assert_eq!("Movigation", state.text());
        assert_eq!(1, state.cursor);

        // Do nothing at END position
        end(&mut state);
        translate_char(&mut state, &table);
        assert_eq!("Movigation", state.text());
        assert_eq!(NAV_LAST, state.cursor);

        last(&mut state);
        translate_char(&mut state, &table);
        assert_eq!("MovigatioM", state.text());
        assert_eq!(NAV_LAST_CHARP, state.cursor);

        // Is NOT idempotent if table contains A -> B and B -> C etc..
        translate_char(&mut state, &table);
        assert_eq!("MovigatioN", state.text());
        assert_eq!(NAV_LAST_CHARP, state.cursor);
        translate_char(&mut state, &table);
        assert_eq!("MovigatioM", state.text());
        assert_eq!(NAV_LAST_CHARP, state.cursor);
        translate_char(&mut state, &table);
        assert_eq!("MovigatioN", state.text());
        assert_eq!(NAV_LAST_CHARP, state.cursor);
    }

    #[test]
    fn transpose_character_cmd() {
        let mut state = LineState::new(SAMPLE);

        forward(&mut state);
        transpose_character(&mut state);
        assert_eq!("anvigation", state.text());
        assert_eq!(2, state.cursor);

        transpose_character(&mut state);
        assert_eq!("avnigation", state.text());
        assert_eq!(3, state.cursor);

        end(&mut state);
        transpose_character(&mut state);
        assert_eq!("avnigatino", state.text());
        assert_eq!(NAV_LAST, state.cursor);

        end(&mut state);
        transpose_character(&mut state);
        assert_eq!("avnigation", state.text());
        assert_eq!(NAV_LAST, state.cursor);

        // Last behaves same as end
        last(&mut state);
        transpose_character(&mut state);
        assert_eq!("avnigatino", state.text());
        assert_eq!(NAV_LAST, state.cursor);
    }
}
