mod layout;

use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{self, DefaultTerminal, Frame};

#[derive(Default)]
pub struct App {
    exit: bool,
}

impl App {
    //takes in terminal and then passes the terminal frame to the functions below
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    //accepts a terminal frame to draw to, then draws to it
    //      called from run() above
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    //called to take care of the event handling
    //      we will implement this with crossterm
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(keypress) = event::read()? {
                match keypress.code {
                    KeyCode::Char('q') => self.exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    //this method is called by the render_widget method call above in self.draw()
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                ratatui::layout::Constraint::Percentage(50),
                ratatui::layout::Constraint::Percentage(50),
            ])
            .spacing(0)
            .margin(0)
            .split(area);

        let left_header = Line::from("STUI Timer").red().centered().bold();
        let left_text = Line::from("Press q to quit").white().centered().not_bold();
        let paragraphs = vec![
            "This is the first paragraph of text.".into(),
            "Here's the second, which will appear on a new line.".into(),
            Line::from(vec![
                "You can even mix styles, like ".into(),
                "bold green text".green().bold(),
                " in the third line!".into(),
            ]),
        ];
        let left_comps: Vec<Line<'_>> = vec![left_header, left_text];
        let block = Block::bordered()
            .borders(!ratatui::widgets::Borders::RIGHT)
            .border_style(Style::new().blue());
        let block2 = Block::bordered()
            .border_style(Style::new().blue().bold())
            .add_modifier(Modifier::BOLD);

        Paragraph::new(paragraphs)
            .block(block)
            .render(layout[0], buf);

        Paragraph::new(left_comps)
            .block(block2)
            .render(layout[1], buf);
    }
}

/******************************************************************
Beginnings of a 'Customize Timer' Implementation

The code below can serve, in the future, as a Ratatui wrapper
in which users can leverage pre-defined 'Pieces' (rename to components)
to customize the 'Bit' (rename to App, Panel, or Interface)

pub struct Bit {
    pieces: Vec<Piece>,
    exit: bool,
}

impl Default for Bit {
    fn default() -> Self {
        Self {
            pieces: vec![Piece {}],
            exit: false,
        }
    }
}

impl Bit {
    //takes in terminal and then passes the terminal frame to the functions below
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    //accepts a terminal frame to draw to, then draws to it
    //      called from run() above
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    //called to take care of the event handling
    //      we will implement this with crossterm
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(keypress) = event::read()? {
                match keypress.code {
                    KeyCode::Char('q') => self.exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

NOTE: Unlike I previously thought, I don't need to define implementation in the render function.
I can define it in one of the above functions with the Frame object in scope, and then use that
to call a Piece's render method.

impl Widget for &Bit {
    //this method is called by the render_widget method call above in self.draw()
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        for piece in &self.pieces {
            // Render each piece into the same buffer/area for now.
            // Later you can split `area` and pass different rects to each piece.
            piece.render(area, buf);
        }
    }
}

#[derive(Clone)]
pub struct Piece {}

impl Piece {
    //methods for creating different pieces (-> Components)
}

impl Widget for &Piece {
    //implement the code for what the piece will look like
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let block = ratatui::widgets::Block::bordered().title("Hello Timer");
        block.render(area, buf);
    }
}
*/
