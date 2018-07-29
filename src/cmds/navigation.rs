use super::super::LineState;
use std::cmp;

pub fn home(state: &mut LineState) {
    state.cursor = 0;
}

pub fn end(state: &mut LineState) {
    state.cursor = state.last_cursor_position();
}

pub fn last(state: &mut LineState) {
    state.cursor = state.last_character_position();
}

pub fn forward(state: &mut LineState) {
    state.cursor = cmp::min(state.cursor + 1, state.last_cursor_position());
}

pub fn goto(state: &mut LineState, pos: usize) {
    state.cursor = cmp::max(0, cmp::min(pos, state.last_cursor_position()));
}

pub fn back(state: &mut LineState) {
    state.cursor = if state.cursor == 0 {
        0
    } else {
        state.cursor - 1
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_NAV: &str = "navigation";
    const NAV_LAST: usize = 10;
    const NAV_LAST_CHARP: usize = 9;

    #[test]
    fn back_tests() {
        let mut state = LineState::new(SAMPLE_NAV);

        // Edge case, can't move beyond beginning of line,
        assert_eq!(0, state.cursor, "|^navigation");
        back(&mut state);
        assert_eq!(0, state.cursor, "|^navigation");

        forward(&mut state);
        forward(&mut state);
        assert_eq!(2, state.cursor, "^na|vigation");
        back(&mut state);
        assert_eq!(1, state.cursor, "n|a^vigation");
    }

    #[test]
    fn forward_tests() {
        let mut state = LineState::new(SAMPLE_NAV);

        // Edge case, can't move beyond end of line,
        end(&mut state);
        assert_eq!(NAV_LAST, state.cursor, "navigation|^");
        forward(&mut state);
        assert_eq!(NAV_LAST, state.cursor, "navigation|^");

        home(&mut state);
        assert_eq!(0, state.cursor, "|navigation^");
        forward(&mut state);
        assert_eq!(1, state.cursor, "^n|avigation");
        forward(&mut state);
        assert_eq!(2, state.cursor, "^na|vigation");
    }

    #[test]
    fn end_cmd() {
        let mut state = LineState::new(SAMPLE_NAV);
        assert_eq!(0, state.cursor, "^|navigation");

        end(&mut state);
        assert_eq!(NAV_LAST, state.cursor, "^navigation|");

        // Moving to end, if already at end does nothing.
        end(&mut state);
        assert_eq!(NAV_LAST, state.cursor, "^navigation|");
    }

    #[test]
    fn last_cmd() {
        let mut state = LineState::new(SAMPLE_NAV);
        assert_eq!(0, state.cursor, "^|navigation");

        last(&mut state);
        assert_eq!(NAV_LAST_CHARP, state.cursor, "^navigatio|n");

        // Moving to last, if already at end does nothing.
        last(&mut state);
        assert_eq!(NAV_LAST_CHARP, state.cursor, "^navigatio|n");
    }

    #[test]
    fn home_cmd() {
        let mut state = LineState::new(SAMPLE_NAV);
        assert_eq!(0, state.cursor, "^|navigation");

        forward(&mut state);
        assert_eq!(1, state.cursor, "^|navigation");
        home(&mut state);
        assert_eq!(0, state.cursor, "|navigation^");

        // Moving home, if already home does nothing.
        home(&mut state);
        assert_eq!(0, state.cursor, "|navigation^");
    }

}
