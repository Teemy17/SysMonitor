use std::{ thread, time::Duration, sync::mpsc };
use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{ Block, Borders, Paragraph },
    layout::{ Layout, Constraint, Direction },
    style::{ Style, Color },
};
use termion::{ raw::IntoRawMode, screen::AlternateScreen };
use sysinfo::{ System, SystemExt, CpuExt };
use ::SysMonitor::exit;

pub fn display_cpu_usage() {
    let mut sys = System::new_all();

    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(AlternateScreen::from(stdout));
    let mut terminal = Terminal::new(backend).unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        exit(tx);
    });

    loop {
        sys.refresh_cpu();

        let cpu_list = sys.cpus();

        let cpu_usage_info: String = cpu_list
            .iter()
            .enumerate()
            .map(|(core_id, cpu)| {
                let usage_percentage = cpu.cpu_usage();
                let bar_length = (usage_percentage * 0.6) as usize; // Adjust the multiplier for bar length
                format!(
                    "Core {}: {:>5.2}% | {}{}\n",
                    core_id + 1,
                    usage_percentage,
                    "#".repeat(bar_length),
                    " ".repeat(60 - bar_length)
                )
            })
            .collect();

        let paragraph = Paragraph::new(cpu_usage_info)
            .block(Block::default().title("CPU Usage").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size());

                f.render_widget(paragraph, chunks[0]);
            })
            .unwrap();

        if let Ok(_) = rx.try_recv() {
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
