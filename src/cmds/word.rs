use super::super::LineState;
use super::character;
use super::region;

pub fn upcase_word(state: &mut LineState) {
    let start = current_word_start(state);
    let end = current_word_end(state);

    region::upcase_sequence(state, start, end);
}

pub fn downcase_word(state: &mut LineState) {
    let start = current_word_start(state);
    let end = current_word_end(state);

    region::downcase_sequence(state, start, end);
}

pub fn sentence_case_word(state: &mut LineState) {
    let pos = current_word_start(state);
    let old = state.cursor;
    state.cursor = pos;
    character::upcase_character(state);
    state.cursor = old;
}

enum WordNavigationState {
    BeforeWord,
    InsideWord,
}

fn is_word_separator(c: char) -> bool {
    match c {
        ' ' | '\t' | '.' | ',' | ';' | ':' => true,
        _ => false,
    }
}

pub fn current_word_start(state: &LineState) -> usize {
    let mut pos = state.cursor;

    // If not at a word, go back to first word.
    while pos > 0 && (pos == state.characters.len() || is_word_separator(state.characters[pos])) {
        pos -= 1;
    }

    // At a word, go back until we are not at a word anymore
    while pos > 0 && !is_word_separator(state.characters[pos - 1]) {
        pos -= 1;
    }

    pos
}

pub fn previous_word_start(state: &LineState) -> usize {
    let mut pos = state.cursor;

    // At a word, go back until we are not at a word anymore
    while pos > 0 && !is_word_separator(state.characters[pos]) {
        pos -= 1;
    }

    // Skip whitespace
    while pos > 0 && (pos == state.characters.len() || is_word_separator(state.characters[pos])) {
        pos -= 1;
    }

    // At a word, go back until we are not at a word anymore
    while pos > 0 && !is_word_separator(state.characters[pos - 1]) {
        pos -= 1;
    }

    pos
}

pub fn current_word_end(state: &LineState) -> usize {
    let mut pos = state.cursor;
    let end = state.last_cursor_position();

    // If not at a word, go to first word.
    while pos < end && is_word_separator(state.characters[pos]) {
        pos += 1;
    }

    // At a word, go forward until we are not at a word anymore
    while pos < end && !is_word_separator(state.characters[pos]) {
        pos += 1;
    }

    pos
}

pub fn forward_word(state: &mut LineState) {
    let mut word_state = WordNavigationState::BeforeWord;
    for cursor in state.cursor..state.characters.len() {
        let c = state.characters.get(cursor);
        if let Some(&c) = c {
            match word_state {
                WordNavigationState::BeforeWord => if !is_word_separator(c) {
                    word_state = WordNavigationState::InsideWord;
                },
                WordNavigationState::InsideWord => if is_word_separator(c) {
                    state.cursor = cursor;
                    return;
                },
            }
        }
    }
}

pub fn back_word(state: &mut LineState) {
    let mut word_state = WordNavigationState::BeforeWord;
    for cursor in (0..=state.cursor).rev() {
        let c = state.characters.get(cursor);
        if let Some(&c) = c {
            match word_state {
                WordNavigationState::BeforeWord => if !is_word_separator(c) {
                    word_state = WordNavigationState::InsideWord;
                },
                WordNavigationState::InsideWord => if is_word_separator(c) {
                    state.cursor = cursor + 1;
                    return;
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::navigation::*;
    use super::*;

    const SAMPLE_BACK: &str = "   Back, ";
    const BACK_LEN: usize = 9;
    const BACK_LAST: usize = 9;

    #[test]
    fn current_word_start_fn() {
        let mut state = LineState::new("Hello my name is.");

        let pos = vec![
            (0, 0),
            (1, 0),
            (5, 0),
            (8, 6),
            (9, 9),
            (10, 9),
            (11, 9),
            (12, 9),
            (13, 9),
            (14, 14),
            (15, 14),
        ];

        for (src, dst) in pos {
            goto(&mut state, src);
            assert_eq!(dst, current_word_start(&state));
        }
    }

    #[test]
    fn current_word_end_fn() {
        let mut state = LineState::new("Hello my name is");

        let pos = vec![(0, 5), (1, 5), (4, 5), (5, 8), (15, 16)];

        for (src, dst) in pos {
            goto(&mut state, src);
            assert_eq!(dst, current_word_end(&state));
        }
    }

    #[test]
    fn forward_word_test() {
        let mut state = LineState::new(SAMPLE_BACK);
        forward_word(&mut state);
        assert_eq!(7, state.cursor, "^   Back|, ");

        state.set(SAMPLE_BACK.chars().collect());
        forward(&mut state);
        forward_word(&mut state);
        assert_eq!(7, state.cursor, " ^  Back|, ");

        state.set(SAMPLE_BACK.chars().collect());
        for _ in 0..3 {
            forward(&mut state);
        }
        forward_word(&mut state);
        assert_eq!(7, state.cursor, "   ^Back|, ");

        state.set(SAMPLE_BACK.chars().collect());
        for _ in 0..4 {
            forward(&mut state);
        }
        forward_word(&mut state);
        assert_eq!(7, state.cursor, "   B^ack|, ");

        state.set(SAMPLE_BACK.chars().collect());
        for _ in 0..7 {
            forward(&mut state);
        }
        forward_word(&mut state);
        assert_eq!(7, state.cursor, "   Bac^k|, ");

        state.set(SAMPLE_BACK.chars().collect());
        for _ in 0..8 {
            forward(&mut state);
        }
        forward_word(&mut state);
        assert_eq!(8, state.cursor, "   Back,^ |");

        state.set(SAMPLE_BACK.chars().collect());
        end(&mut state);
        assert_eq!(BACK_LAST, state.cursor, "   Back, ^|");
        forward_word(&mut state);
        assert_eq!(BACK_LAST, state.cursor, "   Back, ^|");
    }

}
