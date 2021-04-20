use std::io;
extern crate rorg_agenda;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::style::{Style};
use tui::widgets::{Block, Borders,List,ListItem};
use tui::layout::{Layout, Constraint, Direction};

pub fn init() -> Result<(), io::Error> {
	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	terminal.clear()?;
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
		let tasklist = tasklist(file.events);
		f.render_widget(tasklist, chunks[0]);

		let taskview = Block::default()
			.title("Task view")
			.borders(Borders::ALL);
		f.render_widget(taskview, chunks[1]);
	})
}

fn tasklist(event_vector: Vec<rorg_agenda::rorg_types::Event>) -> List<'static>{
	let mut items = Vec::new();

	for event in event_vector{
		items.push(ListItem::new(event.name))
	}

	List::new(items)
		.block(Block::default().title("Task list").borders(Borders::ALL))
		.style(Style::default())
		.highlight_style(Style::default())
		.highlight_symbol(">>")
}
