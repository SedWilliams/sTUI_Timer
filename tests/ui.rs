/*
use insta::assert_snapshot;
use ratatui::{Terminal, backend::TestBackend};
use stui_timer::util::io::ui::app::App;

// NEED TO IMPL WIDGET FOR APP HERE FOR THIS TEST TO WORK

#[test]
#[should_panic]
fn ratatui_ui_test() {
    let app = App::init();
    let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
    terminal
        .draw(|frame| {
            frame.render_widget(&app, frame.area());
        })
        .unwrap();

    assert_snapshot!(terminal.backend());
}
*/
