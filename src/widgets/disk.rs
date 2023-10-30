use std::{ sync::mpsc, thread, time::Duration };
use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{ Block, Borders, Paragraph },
    layout::{ Layout, Constraint, Direction },
};
use termion::{ raw::IntoRawMode, screen::AlternateScreen };
use sysinfo::{ System, SystemExt, DiskExt };
use SysMonitor::exit;

pub fn display_disk_usage() {
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(AlternateScreen::from(stdout));
    let mut terminal = Terminal::new(backend).unwrap();

    let mut sys = System::new_all();
    sys.refresh_disks();

    let disk_block = Block::default().title("Disk Information").borders(Borders::ALL);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        exit(tx);
    });

    loop {
        sys.refresh_disks_list();
        let mut disk_text: Vec<String> = Vec::new();

        for disk in sys.disks() {
            let usage = disk.total_space() - disk.available_space();
            let total = disk.total_space();
            let removable = disk.is_removable();
            let filesystem = convert_ascii_to_text(disk.file_system());
            let disk_kind = disk.kind();
            let spacer = "-".repeat(60);

            let text = format!(
                "Disk: {:?} - {:.2} GB used / {:.2} GB total \n Filesystem: {:?} \n Type: {:?} \n Removeable: {} \n{}\n",
                disk.name(),
                (usage as f64) / 1024.0 / 1024.0 / 1024.0,
                (total as f64) / 1024.0 / 1024.0 / 1024.0,
                filesystem,
                disk_kind,
                removable,
                spacer
            );

            disk_text.push(text);
        }

        let disk_block_clone = disk_block.clone();

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size());

                let disk_paragraph = Paragraph::new(disk_text.join(""))
                    .block(disk_block_clone)
                    .alignment(tui::layout::Alignment::Left);

                f.render_widget(disk_paragraph, chunks[0]);
            })
            .unwrap();

        if let Ok(_) = rx.try_recv() {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
}

//Convert filesystem from ascii to text
fn convert_ascii_to_text(ascii: &[u8]) -> String {
    ascii
        .into_iter()
        .map(|c| *c as char)
        .collect::<String>()
        .trim()
        .to_string()
}
