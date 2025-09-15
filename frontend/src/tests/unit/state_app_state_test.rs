use crate::state::AppState;

#[test]
fn given_new_app_state_when_initialized_then_should_have_no_current_file() {
    let state = AppState::new();
    assert!(state.current_file.is_none());
}
