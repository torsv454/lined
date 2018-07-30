use super::cmds::character::*;
use super::cmds::clipboard::*;
use super::cmds::control::*;
use super::cmds::insert::*;
use super::cmds::kill::*;
use super::cmds::line::*;
use super::cmds::navigation::*;
use super::cmds::region::*;
use super::cmds::search::*;
use super::cmds::word::*;
use std::cmp;
use std::collections::HashMap;
use std::ops::Range;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct LineState {
    pub cursor: usize,
    pub characters: Vec<char>,
    pub mark: Option<usize>,
    pub clipboard: Vec<Vec<char>>,
    pub done_with_line: bool,
}

impl LineState {
    pub fn new(line: &str) -> LineState {
        let characters: Vec<char> = line.chars().collect();

        LineState {
            cursor: 0,
            characters,
            mark: None,
            clipboard: Vec::new(),
            done_with_line: false,
        }
    }

    #[cfg(test)]
    pub fn set(&mut self, characters: Vec<char>) {
        self.cursor = 0;
        self.mark = None;
        self.clipboard.clear();
        self.characters = characters;
        self.done_with_line = false;
    }
    // pub fn after_cursor(&self) -> RangeInclusive<usize> {
    //     self.cursor + 1..=self.cursor + 1
    // }

    pub fn at_cursor(&self) -> RangeInclusive<usize> {
        self.cursor..=self.cursor
    }

    pub fn shift_mark_if_greater(&mut self, left: usize, delta: usize) {
        if let Some(mark) = self.mark {
            if mark > left {
                self.mark = Some(cmp::max(left, mark - delta));
            }
        }
    }

    // pub fn before_cursor(&self) -> RangeInclusive<usize> {
    //     self.cursor - 1..=self.cursor - 1
    // }

    pub fn last_cursor_position(&self) -> usize {
        self.characters.len()
    }

    pub fn last_character_position(&self) -> usize {
        self.characters.len() - 1
    }

    pub fn at_character(&self) -> bool {
        self.cursor < self.characters.len()
    }

    pub fn at_end(&self) -> bool {
        self.cursor == self.characters.len()
    }

    pub fn insertion_point(&self) -> Range<usize> {
        self.cursor..self.cursor
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.characters.iter().collect()
    }

    #[cfg(test)]
    pub fn clipboard_text(&self) -> Option<String> {
        self.clipboard.last().map(|x| x.iter().collect())
    }
}

#[derive(Debug)]
pub enum Cmd {
    // Navigation
    Back,
    Forward,
    ForwardWord,
    BackWord,
    Home,
    End,
    Last,
    Goto(usize),

    // Character commands
    Delete,
    DeleteBefore,
    TransposeCharacter,
    UpcaseCharacter,
    DowncaseCharacter,
    Translate { table: HashMap<char, char> },
    CopyLine,

    // Word commands
    KillWord,
    RKillWord,
    KillFullWord,
    TransposeWord,
    UpcaseWord,
    DowncaseWord,
    SentenceCaseWord,

    // Region commands
    Mark,
    UpcaseRegion,
    DowncaseRegion,
    Transpose,

    // Clipboard
    Copy,
    Paste,
    Cut,
    UpcaseClipboard,
    DowncaseClipboard,
    SentencecaseClipboard,
    LeftTrimClipboard,
    RightTrimClipboard,
    TrimClipboard,

    // Kill
    Kill,
    KillLine,
    RKillLine,
    TruncateBy(usize),
    TrimLine,
    LTrimLine,
    RTrimLine,
    UpcaseLine,
    DowncaseLine,

    // Searching
    Find { what: char },
    RFind { what: char },

    // Other
    NextLine,
    Repeat { times: usize, cmd: Box<Cmd> },
    Insert { what: String },
    Block(Vec<Cmd>),
}

impl Cmd {
    pub fn eval(&self, state: &mut LineState) {
        //!("eval: {:?} when {:?}", self, state);
        match self {
            Cmd::Back => back(state),
            Cmd::Forward => forward(state),
            Cmd::ForwardWord => forward_word(state),
            Cmd::BackWord => back_word(state),
            Cmd::Home => home(state),
            Cmd::End => end(state),
            Cmd::Goto(column) => goto(state, *column),
            Cmd::Last => last(state),
            Cmd::Delete => delete(state),
            Cmd::DeleteBefore => delete_before(state),
            Cmd::Mark => mark(state),
            Cmd::Copy => copy(state),
            Cmd::CopyLine => copy_line(state),
            Cmd::TruncateBy(amount) => truncate_by(state, *amount),
            Cmd::UpcaseRegion => upcase(state),
            Cmd::DowncaseRegion => downcase(state),
            Cmd::UpcaseClipboard => upcase_clipboard(state),
            Cmd::DowncaseClipboard => downcase_clipboard(state),
            Cmd::SentencecaseClipboard => sentencecase_clipboard(state),
            Cmd::LeftTrimClipboard => left_trim_clipboard(state),
            Cmd::RightTrimClipboard => right_trim_clipboard(state),
            Cmd::TrimClipboard => trim_clipboard(state),
            Cmd::DowncaseLine => downcase_line(state),
            Cmd::UpcaseLine => upcase_line(state),
            Cmd::TrimLine => trim_line(state),
            Cmd::LTrimLine => ltrim_line(state),
            Cmd::RTrimLine => rtrim_line(state),
            Cmd::Cut => cut(state),
            Cmd::Paste => paste(state),
            Cmd::Repeat { times, cmd } => repeat(state, *times, cmd),
            Cmd::Insert { what } => insert(&what, state),
            Cmd::Find { what } => state.done_with_line = !find(*what, state),
            Cmd::RFind { what } => state.done_with_line = !rfind(*what, state),
            Cmd::Kill => kill(state),
            Cmd::Transpose => transpose(state),
            Cmd::NextLine => state.done_with_line = true,
            Cmd::Translate { table } => translate(state, table),
            Cmd::Block(ref cmds) => block(cmds, state),
            Cmd::TransposeCharacter => transpose_character(state),
            Cmd::UpcaseCharacter => upcase_character(state),
            Cmd::DowncaseCharacter => downcase_character(state),
            Cmd::KillWord => kill_word(state),
            Cmd::KillFullWord => reverse_kill_word(state),
            Cmd::RKillWord => kill_full_word(state),
            Cmd::TransposeWord => kill_full_word(state),
            Cmd::UpcaseWord => upcase_word(state),
            Cmd::DowncaseWord => downcase_word(state),
            Cmd::SentenceCaseWord => sentence_case_word(state),
            Cmd::KillLine => kill_line_after(state),
            Cmd::RKillLine => kill_line_before(state),
        }
    }
}

pub fn translate(state: &mut LineState, table: &HashMap<char, char>) {
    state.characters.iter_mut().for_each(|c| {
        if let Some(&t) = table.get(c) {
            *c = t;
        }
    });
}

pub fn run(program: &[Cmd], line: &str) -> String {
    let mut state = LineState::new(line);

    for cmd in program {
        cmd.eval(&mut state);
    }

    state.characters.iter().collect()
}
