use termion::input::TermRead;

// Handle the exit key
// In the program a sperate thread is created to handle the exit key.
pub fn exit(tx: std::sync::mpsc::Sender<()>) {
    for c in std::io::stdin().keys() {
        if let Ok(key) = c {
            if key == termion::event::Key::Ctrl('q') {
                if let Err(_) = tx.send(()) {
                    break;
                }
            }
        }
    }
}
