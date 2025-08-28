#[derive(Debug, Clone, PartialEq)]
pub enum CentralView {
    Ctxs,
    Pods,
    Help,
}

pub enum ModeState {
    Sim,
    Kube,
}

pub struct CommandState {
    pub input: String,
    pub view: CentralView,
    pub mode: ModeState,
}

impl CommandState {
    pub fn new() -> Self {
        CommandState {
            input: String::new(),
            view: CentralView::Pods,
            mode: ModeState::Kube,
        }
    }

    pub fn handle_command(&mut self) {
        match self.input.trim() {
            ":togglesim" => match self.mode {
                ModeState::Sim => self.mode = ModeState::Kube,
                ModeState::Kube => self.mode = ModeState::Sim,
            },
            ":pods" => self.view = CentralView::Pods,
            ":ctx" => self.view = CentralView::Ctxs,
            ":help" => self.view = CentralView::Help,
            _ => {},
        }
        self.input.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST: GIVEN a new CommandState WHEN constructed THEN input is empty and view is Pods
    #[test]
    fn test_command_state_new() {
        let state = CommandState::new();
        assert_eq!(state.input, "");
        assert_eq!(state.view, CentralView::Pods);
    }

    // TEST: GIVEN CommandState WHEN :pods command is entered THEN view switches to Pods and input is cleared
    #[test]
    fn test_handle_command_pods() {
        let mut state = CommandState::new();
        state.input = ":pods".to_string();
        state.handle_command();
        assert_eq!(state.view, CentralView::Pods);
        assert_eq!(state.input, "");
    }

    // TEST: GIVEN CommandState WHEN :help command is entered THEN view switches to Help and input is cleared
    #[test]
    fn test_handle_command_help() {
        let mut state = CommandState::new();
        state.input = ":help".to_string();
        state.handle_command();
        assert_eq!(state.view, CentralView::Help);
        assert_eq!(state.input, "");
    }

    // TEST: GIVEN CommandState WHEN :ctx command is entered THEN view switches to Ctxs and input
    // is cleared 
    #[test]
    fn test_handle_command_ctx() {
        let mut state = CommandState::new();
        state.input = ":ctx".to_string();
        state.handle_command();
        assert_eq!(state.view, CentralView::Ctxs);
        assert_eq!(state.input, "");
    }

    // TEST: GIVEN CommandState WHEN unknown command is entered THEN view does not change and input is cleared
    #[test]
    fn test_handle_command_unknown() {
        let mut state = CommandState::new();
        state.input = ":unknown".to_string();
        state.handle_command();
        // Should not change view
        assert_eq!(state.view, CentralView::Pods);
        assert_eq!(state.input, "");
    }
}

