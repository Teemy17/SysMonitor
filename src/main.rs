mod widgets;
use widgets::{
    memory::display_ram_usage,
    sys_information::display_system_information,
    network::display_network_usage,
    cpu::display_cpu_usage,
    disk::display_disk_usage,
    temperature::display_temperature,
};
use clap::Parser;

/// SysMonitor CLI, Display system information in the terminal.
/// To exit the TUI, press the Ctrl + 'q' key.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, verbatim_doc_comment)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Option to display memory usage
    #[arg(short, long)]
    memory: bool,

    /// Option to display system information
    #[arg(short, long)]
    system_info: bool,

    /// Option to display network usage
    #[arg(short, long)]
    network: bool,

    /// Option to display CPU usage
    #[arg(short, long)]
    cpu: bool,

    /// Option to display disk usage
    #[arg(short, long)]
    disk: bool,

    /// Option to display components temperature
    #[arg(short, long)]
    temperature: bool,
}

fn main() {
    let args = Args::parse();

    if args.memory {
        display_ram_usage();
    }

    if args.system_info {
        display_system_information();
    }

    if args.network {
        display_network_usage();
    }

    if args.cpu {
        display_cpu_usage();
    }

    if args.disk {
        display_disk_usage();
    }

    if args.temperature {
        display_temperature();
    }
}
