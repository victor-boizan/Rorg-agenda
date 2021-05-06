extern crate rorg_agenda;

use tui::style::{Style,Modifier};
use tui::widgets::{Block, Borders,List,ListItem,Paragraph,Wrap,ListState};
use tui::layout::Alignment;
use tui::text::{Spans,Span};

pub fn view(event_vector: Vec<rorg_agenda::rorg_types::Event>,eventlist_state: &ListState) -> (List<'static>,Paragraph<'static>) {
	let items: Vec<_> = event_vector
		.iter()
		.map(|event| {
			ListItem::new(Spans::from(vec![Span::styled(
				event.name.clone(),
				Style::default(),
			)]))
		})
		.collect();

	let selected_event = event_vector
	.get(
		eventlist_state
			.selected()
			.expect("Nothing is selected"),
	).unwrap()
	.clone();

	let list = List::new(items)
		.block(Block::default().title("Task list").borders(Borders::ALL))
		.style(Style::default())
		.highlight_style(Style::default())
		.highlight_symbol(">>");

	let text = vec![
		Spans::from(Span::raw(format!("{:?}\n: {}",selected_event.state.unwrap(),selected_event.name))),
		Spans::from(Span::raw("")),
		Spans::from(Span::styled("Descritpion:", Style::default().add_modifier(Modifier::BOLD))),
		Spans::from(Span::raw(selected_event.description.unwrap())),
		Spans::from(Span::raw("")),
		Spans::from(Span::styled("Notes:", Style::default().add_modifier(Modifier::BOLD))),
		Spans::from(Span::raw(selected_event.notes.unwrap())),
	];

	let paragraph = Paragraph::new(text)
		.block(Block::default().title("Task view").borders(Borders::ALL))
		.alignment(Alignment::Left)
		.wrap(Wrap { trim: true });

	(list,paragraph)
}
