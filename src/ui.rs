use std::io;

use tui::Terminal;
use tui::backend::CrosstermBackend;



pub fn init() -> Result<(), io::Error> {
	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	terminal.clear()

}
