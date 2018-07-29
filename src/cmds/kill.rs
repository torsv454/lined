use super::super::LineState;
use super::word;

pub fn kill_line_after(state: &mut LineState) {
    let len = state.characters.len();
    let pos = state.cursor;
    let delta = len - pos;
    state.characters.truncate(state.cursor);
    state.shift_mark_if_greater(pos, delta);
}

pub fn kill_line_before(state: &mut LineState) {
    let _ = state.characters.drain(0..state.cursor).last();
    let pos = state.cursor;
    state.shift_mark_if_greater(0, pos);
    state.cursor = 0;
}

pub fn kill_full_word(state: &mut LineState) {
    let from = word::current_word_start(state);
    let to = word::current_word_end(state);
    let _ = state.characters.drain(from..to).last();
    state.shift_mark_if_greater(from, to - from);
    state.cursor = from;
}

pub fn reverse_kill_word(state: &mut LineState) {
    let to = state.cursor;
    let from = {
        let from = word::current_word_start(state);
        if from == to {
            word::previous_word_start(state)
        } else {
            from
        }
    };
    let _ = state.characters.drain(from..to).last();
    state.shift_mark_if_greater(from, to - from);
    state.cursor = from;
}

pub fn kill_word(state: &mut LineState) {
    let to = word::current_word_end(state);
    let from = state.cursor;
    let _ = state.characters.drain(from..to).last();
    state.shift_mark_if_greater(from, to - from);
    state.cursor = from;
}

pub fn kill(state: &mut LineState) {
    state.characters.truncate(state.cursor);
}

#[cfg(test)]
mod tests {
    use super::super::navigation::*;
    use super::*;

    const SAMPLE: &str = "navigation";
    const NAV_LAST: usize = 10;
    const NAV_LAST_CHARP: usize = 9;

    #[test]
    fn kill_before_cmd() {
        let mut state = LineState::new(SAMPLE);

        // Nothing is removed if at home
        kill_line_before(&mut state);
        assert_eq!("navigation", state.text());

        // Move forward 3, kills first 3
        for _ in 0..3 {
            forward(&mut state);
        }
        kill_line_before(&mut state);
        assert_eq!("igation", state.text());

        // Moving to end kills everything
        end(&mut state);
        kill_line_before(&mut state);
        assert_eq!("", state.text());

        // Moving to last kills everyting except last
        state = LineState::new(SAMPLE);
        last(&mut state);
        kill_line_before(&mut state);
        assert_eq!("n", state.text());
    }

    #[test]
    fn kill_after_cmd() {
        let mut state = LineState::new(SAMPLE);

        // Entire line is removed if at home
        kill_line_after(&mut state);
        assert_eq!("", state.text());

        state = LineState::new(SAMPLE);
        // Move forward 3, kills all but first 3
        for _ in 0..3 {
            forward(&mut state);
        }
        kill_line_after(&mut state);
        assert_eq!("nav", state.text());

        // Moving to end kills nothing
        end(&mut state);
        kill_line_after(&mut state);
        assert_eq!("nav", state.text());

        // Moving to last kills last
        last(&mut state);
        kill_line_after(&mut state);
        assert_eq!("na", state.text());

        // If mark is before left point of deletion it is unaffected

        // If mark is after left point of deletion it is shifted
    }

    #[test]
    fn kill_full_word_cmd() {
        let mut state = LineState::new("");

        fn reset(state: &mut LineState) {
            state.set("Hello my name is.".chars().collect());
            state.mark = Some(11);
        }

        for i in 9..13 {
            reset(&mut state);
            goto(&mut state, i);
            kill_full_word(&mut state);
            assert_eq!("Hello my  is.", state.text());
            assert_eq!(9, state.cursor);
            assert_eq!(Some(9), state.mark);
        }
    }

    #[test]
    fn kill_word_cmd() {
        let mut state = LineState::new("");

        fn reset(state: &mut LineState) {
            state.set("Hello my name is.".chars().collect());
            state.mark = Some(11);
        }

        reset(&mut state);
        goto(&mut state, 9);
        kill_word(&mut state);
        assert_eq!("Hello my  is.", state.text());
        assert_eq!(9, state.cursor);
        assert_eq!(Some(9), state.mark);

        reset(&mut state);
        goto(&mut state, 10);
        kill_word(&mut state);
        assert_eq!("Hello my n is.", state.text());
        assert_eq!(10, state.cursor);
        assert_eq!(Some(10), state.mark);

        reset(&mut state);
        goto(&mut state, 11);
        kill_word(&mut state);
        assert_eq!("Hello my na is.", state.text());
        assert_eq!(11, state.cursor);
        assert_eq!(Some(11), state.mark);

        reset(&mut state);
        goto(&mut state, 12);
        kill_word(&mut state);
        assert_eq!("Hello my nam is.", state.text());
        assert_eq!(12, state.cursor);
        assert_eq!(Some(11), state.mark);

        reset(&mut state);
        goto(&mut state, 13);
        kill_word(&mut state);
        assert_eq!("Hello my name.", state.text());
        assert_eq!(13, state.cursor);
        assert_eq!(Some(11), state.mark);
    }

    #[test]
    fn rkill_word_cmd() {
        let mut state = LineState::new("");

        fn reset(state: &mut LineState) {
            state.set("Hello my name is.".chars().collect());
            state.mark = Some(11);
        }

        reset(&mut state);
        goto(&mut state, 13);
        reverse_kill_word(&mut state);
        assert_eq!("Hello my  is.", state.text());
        assert_eq!(9, state.cursor);
        assert_eq!(Some(9), state.mark);

        reset(&mut state);
        goto(&mut state, 12);
        reverse_kill_word(&mut state);
        assert_eq!("Hello my e is.", state.text());
        assert_eq!(9, state.cursor);
        assert_eq!(Some(9), state.mark);

        reset(&mut state);
        goto(&mut state, 11);
        reverse_kill_word(&mut state);
        assert_eq!("Hello my me is.", state.text());
        assert_eq!(9, state.cursor);
        assert_eq!(Some(9), state.mark);

        reset(&mut state);
        goto(&mut state, 10);
        reverse_kill_word(&mut state);
        assert_eq!("Hello my ame is.", state.text());
        assert_eq!(9, state.cursor);
        assert_eq!(Some(10), state.mark);

        reset(&mut state);
        goto(&mut state, 9);
        reverse_kill_word(&mut state);
        assert_eq!("Hello name is.", state.text());
        assert_eq!(6, state.cursor);
        assert_eq!(Some(8), state.mark);
    }
}
