use std::{ thread, time::Duration };
use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{ Block, Borders, Paragraph, Sparkline },
    layout::{ Layout, Constraint, Direction },
    style::{ Style, Color },
};
use termion::{ raw::IntoRawMode, screen::AlternateScreen };
use sysinfo::{ System, SystemExt, NetworkExt };
use SysMonitor::exit;

pub fn display_network_usage() {
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(AlternateScreen::from(stdout));
    let mut terminal = Terminal::new(backend).unwrap();

    let mut sys = System::new_all();
    sys.refresh_all();

    let network_block = Block::default().title("Network Usage").borders(Borders::ALL);

    let mut received_data: Vec<f64> = Vec::new();
    let mut transmitted_data: Vec<f64> = Vec::new();

    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        exit(tx);
    });

    loop {
        sys.refresh_networks();
        let mut network_text: Vec<String> = Vec::new();

        for (interface_name, interface) in sys.networks() {
            let received_kb = (interface.received() as f64) / 1024.0;
            let transmitted_kb = (interface.transmitted() as f64) / 1024.0;

            received_data.push(received_kb);
            transmitted_data.push(transmitted_kb);

            let text = format!(
                "{}: {:.2} KB/s in, {:.2} KB/s out\n",
                interface_name,
                received_kb,
                transmitted_kb
            );
            network_text.push(text);
        }

        // Trim the sparkline data
        if received_data.len() > 100 {
            received_data = received_data.split_off(received_data.len() - 100);
            transmitted_data = transmitted_data.split_off(transmitted_data.len() - 100);
        }

        let network_block_clone = network_block.clone();

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                    .split(f.size());

                let network_paragraph = Paragraph::new(network_text.join(""))
                    .block(network_block_clone)
                    .style(Style::default().fg(Color::LightCyan))
                    .alignment(tui::layout::Alignment::Left);

                f.render_widget(network_paragraph, chunks[0]);

                // Create datasets for the sparklines
                let received_dataset: Vec<u64> = received_data
                    .iter()
                    .map(|&x| x as u64)
                    .collect();
                let received_sparkline = Sparkline::default()
                    .block(Block::default().title("Received Data").borders(Borders::ALL))
                    .data(&received_dataset);

                f.render_widget(received_sparkline, chunks[1]);

                // Create another sparkline for transmitted data
                let transmitted_dataset: Vec<u64> = transmitted_data
                    .iter()
                    .map(|&x| x as u64)
                    .collect();
                let transmitted_sparkline = Sparkline::default()
                    .block(Block::default().title("Transmitted Data").borders(Borders::ALL))
                    .data(&transmitted_dataset);

                f.render_widget(transmitted_sparkline, chunks[1]);
            })
            .unwrap();

        if let Ok(_) = rx.try_recv() {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
}
