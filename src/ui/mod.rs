use std::io;
use std::io::Write;

use crossterm::{
	terminal,terminal::{
		EnterAlternateScreen,
		LeaveAlternateScreen
	},
	execute,
	event::{
		read,
		poll,
		Event,
		KeyEvent,
		KeyCode,
		KeyModifiers
	}
};

use std::time::Duration;

extern crate rorg_agenda;
mod event_list;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::style::{Style,Color,Modifier};
use tui::widgets::{Block, Borders,List,ListItem,Paragraph,Wrap,ListState};
use tui::layout::{Layout, Constraint, Direction,Alignment};
use tui::text::{Spans,Span};

pub fn init() -> Result<(), io::Error>{
	let mut stdout = io::stdout();
	terminal::enable_raw_mode();
	execute!(stdout, EnterAlternateScreen);
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	let mut eventlist_state = ListState::default();
	eventlist_state.select(Some(0));


	let quit_event = KeyEvent::new(KeyCode::Char('q'),KeyModifiers::NONE);
	let up_event = KeyEvent::new(KeyCode::Up,KeyModifiers::NONE);
	let down_event = KeyEvent::new(KeyCode::Down,KeyModifiers::NONE);

	loop {
		terminal.draw(|f| {
			let chunks = Layout::default()
				.direction(Direction::Horizontal)
				.margin(1)
				.constraints(
					[
						Constraint::Length(20),
						Constraint::Percentage(100)
					].as_ref()
				)
				.split(f.size());
			let file = rorg_agenda::rorg_types::RorgFile::from_file("./rorg/current/2021.org");
			let events = file.events.clone();

			let selected_event = file.events[eventlist_state.selected().unwrap()].clone();

			let (list,paragraph) = event_list::view(events,&eventlist_state);

			f.render_stateful_widget(list, chunks[0],&mut eventlist_state);

			f.render_widget(paragraph, chunks[1]);
		});
		if poll(Duration::from_millis(500)).unwrap() {
			match read().unwrap() {
				Event::Key(event) => {
				if event == quit_event{break}
				else if event == up_event{ eventlist_state.select(Some(eventlist_state.selected().unwrap() - 1))}
				else if event == down_event{ eventlist_state.select(Some(eventlist_state.selected().unwrap() + 1))}
				},
				_ => {}
			}
		}
	}
	execute!(io::stdout(), LeaveAlternateScreen);
	Ok(())
}
