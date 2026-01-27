use std::time;

use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

pub struct Timer {
    start_time: time::Instant,
    elapsed: time::Duration,
}

impl Timer {
    pub fn new() -> Self {
        let initial_time = time::Instant::now();

        Timer {
            start_time: initial_time,
            elapsed: initial_time.elapsed(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = time::Instant::now();
    }

    pub fn update(&mut self) {
        self.elapsed = self.start_time.elapsed();
    }

    pub fn close(&mut self) {
        unimplemented!();
    }

    /*
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        loop {
            if event::poll(time::Duration::from_millis(100))
                .expect("Crossterm error, run 'cargo fetch'")
            {
                if let Event::Key(key) = event::read().expect("Crossterm error, run 'cargo fetch'")
                {
                    if key.code == KeyCode::Char('s') {
                        println!("\n\rTimer stopped.");
                        println!("");
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }
    */
}

impl Widget for &Timer {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        // Implement rendering logic for the timer here
        Paragraph::new(format!("Elapsed time: {} seconds", self.elapsed.as_secs()))
            .centered()
            .render(area, buf);
    }
}
