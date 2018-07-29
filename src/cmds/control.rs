use super::super::Cmd;
use super::super::LineState;

pub fn block(cmds: &Vec<Cmd>, state: &mut LineState) {
    cmds.iter().for_each(|cmd| cmd.eval(state));
}

pub fn repeat(state: &mut LineState, times: usize, cmd: &Box<Cmd>) {
    for _ in 0..times {
        cmd.eval(state);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Cmd;
    use super::*;

    const SAMPLE_NAV: &str = "navigation";

    #[test]
    fn block_cmd() {
        let mut state = LineState::new(SAMPLE_NAV);
        assert_eq!(0, state.cursor, "|^navigation");

        let cmds = vec![Cmd::Forward, Cmd::Forward, Cmd::Forward];
        block(&cmds, &mut state);
        assert_eq!(3, state.cursor, "^nav|igation");
    }

    #[test]
    fn repeat_cmd() {
        let mut state = LineState::new(SAMPLE_NAV);
        assert_eq!(0, state.cursor, "|^navigation");

        repeat(&mut state, 5, &Box::new(Cmd::Forward));
        assert_eq!(5, state.cursor, "^navig|ation");
    }
}
