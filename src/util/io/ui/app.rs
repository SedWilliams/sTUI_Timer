use ratatui::prelude::*;
use ratatui::widgets::{Block, Widget};
use ratatui::{self, DefaultTerminal, Frame, layout::Constraint};
use tui_big_text::{BigText, PixelSize};

use crate::util::io::event::await_choice;
use crate::util::io::ui::components::timer::Timer;
use crate::util::types::TerminalEventReader;

pub struct App {
    timer: Timer,
    timer_running: bool,
    exit: bool,
}

impl App {
    pub fn init() -> Self {
        App {
            //preload timer instance
            timer: Timer::new(),
            timer_running: false,
            exit: false,
        }
    }

    //takes in terminal and then passes the terminal frame to the functions below
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            self.timer.update();
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    //accepts a terminal frame to draw to, then draws to it
    //      called from run() above
    pub fn draw(&self, frame: &mut Frame) {
        let term_area = Rect::new(0, 0, frame.area().width, frame.area().height);

        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Percentage(30),
                Constraint::Min(10),
                Constraint::Percentage(50),
            ])
            .spacing(0)
            .margin(0)
            .split(term_area);

        let header = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().white().bold())
            .lines(vec!["STUI Timer".into()])
            .centered()
            .build();

        let commands = Line::from(vec![
            " Start/Stop<s> ".into(),
            "Quit<q> ".into(),
            "Logs<v> ".into(),
        ]);

        let terminal_outline = Block::bordered()
            .blue()
            .bold()
            .title_bottom(commands.centered());

        header.render(layout[1], frame.buffer_mut());
        terminal_outline.render(term_area, frame.buffer_mut());

        if self.timer_running {
            self.timer.render(layout[3], frame.buffer_mut());
        }
    }

    //called to take care of the event handling
    //      we will implement this with crossterm
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        let reader = TerminalEventReader::new();

        let result = await_choice(&reader).unwrap();

        if &result == "q" {
            self.exit = true;
        } else if &result == "s" && !self.timer_running {
            self.timer.start();
            self.timer_running = true;
        } else if &result == "s" && self.timer_running {
            self.timer = Timer::new();
            self.timer_running = false;
        } else if &result == "v" {
            //view logs logic here
        } else {
            //self.timer.update();
        }

        Ok(())
    }
}
