use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{ Block, Borders, Paragraph },
    layout::{ Layout, Constraint, Direction },
    text::Text,
};
use termion::{ raw::IntoRawMode, screen::AlternateScreen };
use sysinfo::{ System, SystemExt, CpuExt };
use std::io;
use SysMonitor::exit;

pub fn display_system_information() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(AlternateScreen::from(stdout));
    let mut terminal = Terminal::new(backend).unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        exit(tx);
    });

    loop {
        sys.refresh_all();

        let system_name = sys.name().unwrap();
        let kernel_version = sys.kernel_version().unwrap();
        let os_version = sys.long_os_version().unwrap();
        let host_name = sys.host_name().unwrap();
        let uptime = sys.uptime();
        let (hours, minute) = convert_uptime(uptime);
        let cpu = sys.cpus().first().unwrap();
        let memory = sys.total_memory();

        let system_info = Text::from(
            format!(
                "System name: {}\nKernel version: {}\nOS version: {}\nHost name: {}\nUptime: {} hours, {} minutes\nCPU: {}\nMemory: {:.2} GB",
                system_name,
                kernel_version,
                os_version,
                host_name,
                hours,
                minute,
                cpu.brand(),
                (memory as f64) / 1024.0 / 1024.0 / 1024.0
            )
        );

        let paragraph = Paragraph::new(system_info).block(
            Block::default().title("System Information").borders(Borders::ALL)
        );

        terminal
            .draw(|f| {
                // Render the Paragraph widget
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size()); // Use the terminal size

                f.render_widget(paragraph, chunks[0]);
            })
            .unwrap();

        if let Ok(_) = rx.try_recv() {
            break;
        }
    }
}

fn convert_uptime(uptime: u64) -> (u64, u64) {
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    (hours, minutes)
}
