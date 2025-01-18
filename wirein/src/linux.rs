use procfs::net::{tcp, udp};
use procfs::process::{all_processes, Process};

fn main() {
    println!("Custom Netstat Replacement\n");
    println!("{:<10} {:<20} {:<20} {:<10}", "Protocol", "Local Address", "Remote Address", "PID/Program");

    // TCP-Sockets
    if let Ok(tcp_sockets) = tcp() {
        tcp_sockets
            .iter()
            .filter_map(|socket| {
                Some((
                    "TCP",
                    socket.local_address?,
                    socket.remote_address?,
                    get_process_info(socket.inode),
                ))
            })
            .for_each(|(protocol, local, remote, process)| {
                println!(
                    "{:<10} {:<20} {:<20} {:<10}",
                    protocol,
                    format!("{}", local),
                    format!("{}", remote),
                    process
                );
            });
    }

    // UDP-Sockets
    if let Ok(udp_sockets) = udp() {
        udp_sockets
            .iter()
            .filter_map(|socket| {
                Some((
                    "UDP",
                    socket.local_address?,
                    socket.remote_address.unwrap_or_default(), // UDP hat evtl. keine Remote-Adresse
                    get_process_info(socket.inode),
                ))
            })
            .for_each(|(protocol, local, remote, process)| {
                println!(
                    "{:<10} {:<20} {:<20} {:<10}",
                    protocol,
                    format!("{}", local),
                    format!("{}", remote),
                    process
                );
            });
    }
}

// Hilfsfunktion, um Prozessinformationen anhand der Inode zu finden
fn get_process_info(inode: u64) -> String {
    all_processes()
        .ok()
        .into_iter()
        .flat_map(|processes| processes)
        .filter_map(|process| match_process_to_inode(process, inode))
        .next()
        .unwrap_or_else(|| "Unknown".to_string())
}

// Überprüft, ob ein Prozess zu einer Inode passt
fn match_process_to_inode(process: Process, inode: u64) -> Option<String> {
    process.fd().ok()?.iter().find_map(|fd| {
        fd.as_ref().ok().and_then(|fd_info| {
            if fd_info.is_socket() && fd_info.socket_inode() == Some(inode) {
                Some(format!("{} ({})", process.stat.comm, process.stat.pid))
            } else {
                None
            }
        })
    })
}
