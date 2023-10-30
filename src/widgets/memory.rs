use std::{ thread, time::Duration };
use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{ Block, Borders, Gauge, Row, Table },
    layout::{ Layout, Constraint, Direction },
};
use termion::{ raw::IntoRawMode, screen::AlternateScreen };
use sysinfo::{ System, SystemExt };
use SysMonitor::exit;

pub fn display_ram_usage() {
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(AlternateScreen::from(stdout)); // Use AlternateScreen
    let mut terminal = Terminal::new(backend).unwrap();

    // Create a gauge widget to display memory usage
    let ram_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(tui::style::Style::default().fg(tui::style::Color::Yellow))
        .label("0%")
        .percent(0);

    let mut sys = System::new_all();

    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        exit(tx);
    });

    loop {
        sys.refresh_all();

        let ram_usage = get_ram_usage();

        let ram_gauge = ram_gauge
            .clone()
            .percent(ram_usage.into())
            .label(format!("{}%", ram_usage));

        let text = vec![
            format!("Total memory: {:.2} GB", convert_kb_to_gb(sys.total_memory())),
            format!("Free memory : {:.2} GB", convert_kb_to_gb(sys.free_memory())),
            format!("Used memory : {:.2} GB", convert_kb_to_gb(sys.used_memory())),
            format!("Total swap  : {:.2} GB", convert_kb_to_gb(sys.total_swap())),
            format!("Free swap   : {:.2} GB", convert_kb_to_gb(sys.free_swap())),
            format!("Used swap   : {:.2} GB", convert_kb_to_gb(sys.used_swap()))
        ];

        let rows: Vec<Row> = text
            .iter()
            .map(|line| Row::new(vec![line.to_string()]))
            .collect();

        let table = Table::new(rows)
            .block(Block::default().title("Memory Details").borders(Borders::ALL))
            .widths(&[Constraint::Percentage(100)]);

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                    .split(f.size());
                f.render_widget(ram_gauge, chunks[0]);
                f.render_widget(table, chunks[1]);
            })
            .unwrap();

        if let Ok(_) = rx.try_recv() {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn get_ram_usage() -> u8 {
    let mut sys = System::new_all();
    sys.refresh_all();
    let ram_time = sys.used_memory();
    let total_time = sys.total_memory();

    // Calculate memory usage as a percentage
    let ram_usage = (((ram_time as f64) / (total_time as f64)) * 100.0) as u8;

    ram_usage
}

fn convert_kb_to_gb(kb: u64) -> f64 {
    (kb as f64) / 1024.0 / 1024.0 / 1024.0
}
