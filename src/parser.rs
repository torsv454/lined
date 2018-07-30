use cmd::Cmd;
use tokenizer::Token;
use tokenizer::Tokenizer;

const KW_FORWARD: &str = "forward";
const KW_FORWARD_WORD: &str = "forward_word";
const KW_BACK_WORD: &str = "back_word";
const KW_KILL: &str = "kill";
const KW_BACK: &str = "back";
const KW_TRANSPOSE: &str = "transpose";
const KW_NEXTLINE: &str = "nextline";
const KW_MARK: &str = "mark";
const KW_COPY: &str = "copy";
const KW_CUT: &str = "cut";
const KW_HOME: &str = "home";
const KW_END: &str = "end";
const KW_LAST: &str = "last";
const KW_PASTE: &str = "paste";
const KW_DELETE: &str = "delete";
const KW_DELETEBEFORE: &str = "rdelete";
const KW_INSERT: &str = "insert";
const KW_REPEAT: &str = "repeat";
const KW_UPCASE: &str = "upcase";
const KW_DOWNCASE: &str = "downcase";
const KW_UPCASE_CHAR: &str = "upcase_char";
const KW_DOWNCASE_CHAR: &str = "downcase_char";
const KW_TRANSPOSE_CHAR: &str = "transpose_char";
const KW_UPCASE_WORD: &str = "upcase_word";
const KW_DOWNCASE_WORD: &str = "downcase_word";
const KW_SENTENCECASE_WORD: &str = "sentencecase_word";
const KW_TRANSPOSE_WORD: &str = "transpose_word";
const KW_UPCASE_CLIPBOARD: &str = "upcase_clipboard";
const KW_DOWNCASE_CLIPBOARD: &str = "downcase_clipboard";
const KW_SENTENCE_CASE_CLIPBOARD: &str = "sentencecase_clipboard";
const KW_LTRIM_CLIPBOARD: &str = "ltrim_clipboard";
const KW_RTRIM_CLIPBOARD: &str = "rtrim_clipboard";
const KW_TRIM_CLIPBOARD: &str = "trim_clipboard";
const KW_KILL_WORD: &str = "kill_word";
const KW_RKILL_WORD: &str = "rkill_word";
const KW_KILL_FULL_WORD: &str = "kill_full_word";
const KW_KILL_LINE: &str = "kill_line";
const KW_COPY_LINE: &str = "copy_line";
const KW_RKILL_LINE: &str = "rkill_line";
const KW_TRUNCATE_BY: &str = "truncate_by";
const KW_UPCASE_LINE: &str = "upcase_line";
const KW_DOWNCASE_LINE: &str = "downcase_line";
const KW_LTRIM_LINE: &str = "ltrim_line";
const KW_RTRIM_LINE: &str = "rtrim_line";
const KW_TRIM_LINE: &str = "trim_line";


#[derive(Debug, PartialEq)]
enum ParseError {
    ExpectedString,
    ExpectedNumber,
    ExpectedCommand,
    UnexpectedToken(Token),
}

fn expect_string(tokenizer: &mut Tokenizer) -> Result<String, ParseError> {
    if let Some(Token::STRING(text)) = tokenizer.next() {
        Ok(text)
    } else {
        Err(ParseError::ExpectedString)
    }
}

fn expect_number(tokenizer: &mut Tokenizer) -> Result<i32, ParseError> {
    if let Some(Token::NUM(num)) = tokenizer.next() {
        Ok(num)
    } else {
        Err(ParseError::ExpectedNumber)
    }
}

fn parse_cmd(tokenizer: &mut Tokenizer) -> Result<Option<Cmd>, ParseError> {
    if let Some(token) = tokenizer.next() {
        let cmd = match token {
            Token::WORD(word) => match word.as_ref() {
                KW_FORWARD => Cmd::Forward,
                KW_BACK => Cmd::Back,
                KW_FORWARD_WORD => Cmd::ForwardWord,
                KW_BACK_WORD => Cmd::BackWord,
                KW_TRANSPOSE => Cmd::Transpose,
                KW_TRANSPOSE_CHAR => Cmd::TransposeCharacter,
                KW_UPCASE_CHAR => Cmd::UpcaseCharacter,
                KW_DOWNCASE_CHAR => Cmd::DowncaseCharacter,
                KW_UPCASE_LINE => Cmd::UpcaseLine,
                KW_COPY_LINE => Cmd::CopyLine,
                KW_TRIM_LINE => Cmd::TrimLine,
                KW_LTRIM_LINE => Cmd::LTrimLine,
                KW_RTRIM_LINE => Cmd::RTrimLine,
                KW_UPCASE_CLIPBOARD => Cmd::UpcaseClipboard,
                KW_TRIM_CLIPBOARD => Cmd::TrimClipboard,
                KW_LTRIM_CLIPBOARD => Cmd::LeftTrimClipboard,
                KW_RTRIM_CLIPBOARD => Cmd::RightTrimClipboard,
                KW_DOWNCASE_CLIPBOARD => Cmd::DowncaseClipboard,
                KW_DOWNCASE_LINE => Cmd::DowncaseLine,
                KW_SENTENCE_CASE_CLIPBOARD => Cmd::SentencecaseClipboard,
                KW_TRANSPOSE_WORD => Cmd::TransposeWord,
                KW_UPCASE_WORD => Cmd::UpcaseWord,
                KW_KILL_WORD => Cmd::KillWord,
                KW_RKILL_WORD => Cmd::RKillWord,
                KW_KILL_FULL_WORD => Cmd::KillFullWord,
                KW_SENTENCECASE_WORD => Cmd::SentenceCaseWord,
                KW_DOWNCASE_WORD => Cmd::DowncaseWord,
                KW_NEXTLINE => Cmd::NextLine,
                KW_KILL_LINE => Cmd::KillLine,
                KW_RKILL_LINE => Cmd::RKillLine,
                KW_MARK => Cmd::Mark,
                KW_COPY => Cmd::Copy,
                KW_CUT => Cmd::Cut,
                KW_HOME => Cmd::Home,
                KW_END => Cmd::End,
                KW_LAST => Cmd::Last,
                KW_PASTE => Cmd::Paste,
                KW_DELETE => Cmd::Delete,
                KW_DELETEBEFORE => Cmd::DeleteBefore,
                KW_UPCASE => Cmd::UpcaseRegion,
                KW_DOWNCASE => Cmd::DowncaseRegion,
                KW_KILL => Cmd::Kill,
                KW_TRUNCATE_BY => {
                    let amount = expect_number(tokenizer)? as usize; // TODO: fix me
                    Cmd::TruncateBy(amount)
                }
                KW_REPEAT => {
                    let times = expect_number(tokenizer)? as usize; // TODO: fix me
                    let cmd = parse_cmd(tokenizer)?;
                    if let Some(cmd) = cmd {
                        Cmd::Repeat {
                            times,
                            cmd: Box::new(cmd),
                        }
                    } else {
                        return Err(ParseError::ExpectedCommand);
                    }
                }
                KW_INSERT => Cmd::Insert {
                    what: expect_string(tokenizer)?,
                },
                // KW_FIND => {
                //     let text = expect_string(tokenizer).expect("Expected a string.");
                //     cmds.push(Cmd::Find{what: text});
                // }
                // KW_RFIND => {
                //     let text = expect_string(tokenizer).expect("Expected a string.");
                //     cmds.push(Cmd::RFind{what: text});
                // }
                _ => return Err(ParseError::UnexpectedToken(Token::WORD(word))),
            },
            _ => return Err(ParseError::UnexpectedToken(token)),
        };
        Ok(Some(cmd))
    } else {
        Ok(None)
    }
}

pub fn parse(tokenizer: &mut Tokenizer) -> Option<Vec<Cmd>> {
    let mut cmds = Vec::new();
    loop {
        let cmd = parse_cmd(tokenizer);
        match cmd {
            Ok(Some(cmd)) => cmds.push(cmd),
            Ok(None) => break,
            Err(err) => panic!("Invalid syntax {:?}.", err),
        }
    }

    if cmds.is_empty() {
        None
    } else {
        println!("Cmds: {:?}",cmds);
        Some(cmds)
    }
}
