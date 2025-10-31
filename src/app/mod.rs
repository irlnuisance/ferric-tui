use crate::event::{Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
};

pub mod cmd;
pub mod msg;
pub mod state;
pub mod update;

use msg::Msg;
use state::Model;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub model: Model,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            model: Model::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self.model, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => {
                    self.dispatch(Msg::Tick).await?;
                }
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key) if key.kind == KeyEventKind::Press => {
                        if let Some(msg) = self.map_key_to_msg(key) {
                            self.dispatch(msg).await?;
                        } else {
                            self.dispatch(Msg::Key(key)).await?;
                        }
                    }
                    _ => {}
                },
                Event::App(app_msg) => {
                    self.dispatch(app_msg).await?;
                }
            }
        }
        Ok(())
    }

    async fn dispatch(&mut self, msg: Msg) -> color_eyre::Result<()> {
        if matches!(msg, Msg::Quit) {
            self.running = false;
            return Ok(());
        }
        let (model, cmds) = update::update(std::mem::take(&mut self.model), msg);
        self.model = model;
        if !cmds.is_empty() {
            let tx = self.events.sender_clone();
            crate::app::cmd::spawn_all(cmds, tx);
        }
        Ok(())
    }

    fn map_key_to_msg(&self, key: KeyEvent) -> Option<Msg> {
        match key.code {
            KeyCode::Esc => Some(Msg::Back),
            KeyCode::Char('q') => Some(Msg::Quit),
            KeyCode::Char('c' | 'C') if key.modifiers == KeyModifiers::CONTROL => Some(Msg::Quit),
            KeyCode::Tab => Some(Msg::NextScreen),
            KeyCode::BackTab => Some(Msg::PrevScreen),
            _ => None,
        }
    }
}
