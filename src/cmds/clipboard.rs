use super::super::LineState;

use super::region;

pub fn mark(state: &mut LineState) {
    state.mark = Some(state.cursor);
}

pub fn copy(state: &mut LineState) {
    let region::Region { start, end } = region::region(state);
    state
        .clipboard
        .push(state.characters[start..end].iter().cloned().collect());
}

pub fn cut(state: &mut LineState) {
    let region::Region { start, end } = region::region(state);
    let text: Vec<char> = state.characters.drain(start..end).collect();
    state.clipboard.push(text);
    if state.cursor < end {
        state.mark = Some(start);
    }
    state.cursor = start;
}

pub fn paste(state: &mut LineState) {
    if let Some(text) = state.clipboard.last() {
        let range = state.insertion_point();
        let contents = text.clone();
        state.cursor += contents.len();
        state.characters.splice(range, contents);
    }
}

pub fn upcase_clipboard(state: &mut LineState) {
    if let Some(text) = state.clipboard.pop() {
        state
            .clipboard
            .push(text.iter().flat_map(|c| c.to_uppercase()).collect());
    }
}

pub fn downcase_clipboard(state: &mut LineState) {
    if let Some(text) = state.clipboard.pop() {
        state
            .clipboard
            .push(text.iter().flat_map(|c| c.to_lowercase()).collect());
    }
}

pub fn sentencecase_clipboard(state: &mut LineState) {
    if let Some(mut text) = state.clipboard.pop() {
        if !text.is_empty() {
            let replacement = text.remove(0).to_uppercase();
            text.splice(0..0, replacement);
        }
        state.clipboard.push(text);
    }
}

fn left_remove_until(v: &mut Vec<char>, pred: &Fn(&char) -> bool) {
    let index = v.iter().position(pred).unwrap_or(v.len());
    v.drain(0..index);
}

fn right_remove_until(v: &mut Vec<char>, pred: &Fn(&char) -> bool) {
    let index = v.len() - v.iter().rev().position(pred).unwrap_or(0);
    v.truncate(index);
}

pub fn left_trim_clipboard(state: &mut LineState) {
    if let Some(mut text) = state.clipboard.pop() {
        left_remove_until(&mut text, &|c: &char| !c.is_whitespace());
        state.clipboard.push(text);
    }
}

pub fn right_trim_clipboard(state: &mut LineState) {
    if let Some(mut text) = state.clipboard.pop() {
        right_remove_until(&mut text, &|c: &char| !c.is_whitespace());
        state.clipboard.push(text);
    }
}

pub fn trim_clipboard(state: &mut LineState) {
    if let Some(mut text) = state.clipboard.pop() {
        left_remove_until(&mut text, &|c: &char| !c.is_whitespace());
        right_remove_until(&mut text, &|c: &char| !c.is_whitespace());
        state.clipboard.push(text);
    }
}

#[cfg(test)]
mod tests {
    use super::super::navigation::*;
    use super::*;

    const SAMPLE_BACK: &str = "   Back, ";
    const BACK_LEN: usize = 9;
    const BACK_LAST: usize = 9;
    const SAMPLE_NAV: &str = "navigation";
    const NAV_LEN: usize = 10;
    const NAV_LAST: usize = 10;
    const NAV_LAST_CHARP: usize = 9;

    #[test]
    fn mark_cmd() {
        let mut state = LineState::new(SAMPLE_NAV);
        // No mark with a new state.
        assert_eq!(None, state.mark);

        mark(&mut state);
        assert_eq!(Some(state.cursor), state.mark);

        // Mark remains when moving forward.
        forward(&mut state);
        assert_ne!(Some(state.cursor), state.mark);
        assert_eq!(Some(state.cursor - 1), state.mark);
    }

    #[test]
    fn copy_cmd() {
        let mut state = LineState::new(SAMPLE_NAV);

        // With no mark active, copies nothing
        forward(&mut state);
        copy(&mut state);
        assert_eq!(Some("".to_owned()), state.clipboard_text());

        // With mark active, but on same character, copies nothing
        mark(&mut state);
        copy(&mut state);
        assert_eq!(Some("".to_owned()), state.clipboard_text());

        // Copies first character
        home(&mut state);
        mark(&mut state);
        forward(&mut state);
        copy(&mut state);
        assert_eq!(Some("n".to_owned()), state.clipboard_text());

        // Copy two characters
        forward(&mut state);
        copy(&mut state);
        assert_eq!(Some("na".to_owned()), state.clipboard_text());

        // If mark < cursor or cursor < mark doesn't matter
        mark(&mut state);
        home(&mut state);
        copy(&mut state);
        assert_eq!(Some("na".to_owned()), state.clipboard_text());

        // Copy from end copies entire text
        home(&mut state);
        mark(&mut state);
        end(&mut state);
        copy(&mut state);
        assert_eq!(Some("navigation".to_owned()), state.clipboard_text());

        // Copy from last copies all but last character
        last(&mut state);
        copy(&mut state);
        assert_eq!(Some("navigatio".to_owned()), state.clipboard_text());
    }

    #[test]
    fn cut_cmd() {
        let mut state = LineState::new(SAMPLE_NAV);

        // With no mark active, cuts nothing
        forward(&mut state);
        cut(&mut state);
        assert_eq!(Some("".to_owned()), state.clipboard_text());
        assert_eq!("navigation", state.text());
        assert_eq!(1, state.cursor);

        // With mark active, but on same character, cuts nothing
        mark(&mut state);
        cut(&mut state);
        assert_eq!(Some("".to_owned()), state.clipboard_text());
        assert_eq!("navigation", state.text());
        assert_eq!(1, state.cursor);

        // Cuts first character
        home(&mut state);
        mark(&mut state);
        forward(&mut state);
        cut(&mut state);
        assert_eq!(Some("n".to_owned()), state.clipboard_text());
        assert_eq!("avigation", state.text());
        assert_eq!(0, state.cursor);
        assert_eq!(Some(0), state.mark);

        // Cut two characters
        forward(&mut state);
        forward(&mut state);
        cut(&mut state);
        assert_eq!(Some("av".to_owned()), state.clipboard_text());
        assert_eq!("igation", state.text());
        assert_eq!(0, state.cursor);
        assert_eq!(Some(0), state.mark);

        // If mark < cursor or cursor < mark doesn't matter, except if cursor < mark, then mark is decreased
        forward(&mut state);
        forward(&mut state);
        mark(&mut state);
        home(&mut state);
        cut(&mut state);
        assert_eq!(Some("ig".to_owned()), state.clipboard_text());
        assert_eq!("ation", state.text());
        assert_eq!(0, state.cursor);
        assert_eq!(Some(0), state.mark);

        // Cut from end cuts entire text
        home(&mut state);
        mark(&mut state);
        end(&mut state);
        cut(&mut state);
        assert_eq!(Some("ation".to_owned()), state.clipboard_text());
        assert_eq!("", state.text());
        assert_eq!(0, state.cursor);
        assert_eq!(Some(0), state.mark);
    }

    fn copy_all(state: &mut LineState) {
        home(state);
        mark(state);
        end(state);
        copy(state);
    }

    #[test]
    fn case_clipboard_cmds() {
        let mut state = LineState::new(SAMPLE_NAV);
        copy_all(&mut state);
        upcase_clipboard(&mut state);
        assert_eq!(Some("NAVIGATION".to_owned()), state.clipboard_text());

        downcase_clipboard(&mut state);
        assert_eq!(Some("navigation".to_owned()), state.clipboard_text());

        sentencecase_clipboard(&mut state);
        assert_eq!(Some("Navigation".to_owned()), state.clipboard_text());
    }

    #[test]
    fn trim_clipboard_cmds() {
        const TEXT: &str = "    abc       ";
        let mut state = LineState::new(TEXT);
        copy_all(&mut state);
        left_trim_clipboard(&mut state);
        assert_eq!(Some("abc       ".to_owned()), state.clipboard_text());

        right_trim_clipboard(&mut state);
        assert_eq!(Some("abc".to_owned()), state.clipboard_text());

        copy_all(&mut state);
        assert_eq!(Some(TEXT.to_owned()), state.clipboard_text());
        trim_clipboard(&mut state);
        assert_eq!(Some("abc".to_owned()), state.clipboard_text());
    }

}
