use std::io;
extern crate rorg_agenda;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::style::{Style,Color,Modifier};
use tui::widgets::{Block, Borders,List,ListItem,Paragraph,Wrap};
use tui::layout::{Layout, Constraint, Direction,Alignment};
use tui::text::{Spans,Span};

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
		let events = file.events.clone();
		let selected_event = file.events[0].clone();
		//let tasklist = tasklist(file.events);
		f.render_widget(tasklist(events), chunks[0]);

		f.render_widget(taskview(selected_event), chunks[1]);
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
fn taskview(event: rorg_agenda::rorg_types::Event) -> Paragraph<'static>{

	let text = vec![
		Spans::from(Span::raw(format!("{:?}({}): {}",event.state.unwrap(),event.priority.unwrap(),event.name))),
		Spans::from(Span::raw("")),
		Spans::from(Span::styled("Descritpion:", Style::default().add_modifier(Modifier::BOLD))),
		Spans::from(Span::raw(event.description.unwrap())),
		Spans::from(Span::raw("")),
		Spans::from(Span::styled("Notes:", Style::default().add_modifier(Modifier::BOLD))),
		Spans::from(Span::raw(event.notes.unwrap())),
	];
	Paragraph::new(text)
		.block(Block::default().title("Task view").borders(Borders::ALL))
		.alignment(Alignment::Left)
		.wrap(Wrap { trim: true })
}
