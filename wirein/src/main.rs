use std::collections::HashMap;
use windows::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, GetExtendedUdpTable, TCP_TABLE_OWNER_PID_ALL, UDP_TABLE_OWNER_PID,
    MIB_TCPROW_OWNER_PID, MIB_UDPROW_OWNER_PID,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use std::ptr;

fn main() {
    println!("Custom Netstat Replacement for Windows\n");
    println!("{:<10} {:<20} {:<20} {:<10}", "Protocol", "Local Address", "Remote Address", "PID");

    // TCP-Verbindungen abrufen und anzeigen
    if let Ok(tcp_connections) = get_tcp_connections() {
        tcp_connections.into_iter().for_each(|(local, remote, pid)| {
            println!(
                "{:<10} {:<20} {:<20} {:<10}",
                "TCP",
                local,
                remote,
                pid
            );
        });
    }

    // UDP-Verbindungen abrufen und anzeigen
    if let Ok(udp_connections) = get_udp_connections() {
        udp_connections.into_iter().for_each(|(local, pid)| {
            println!(
                "{:<10} {:<20} {:<20} {:<10}",
                "UDP",
                local,
                "0.0.0.0:0", // UDP hat keine Remote-Adresse
                pid
            );
        });
    }
}

// Funktion, um TCP-Verbindungen abzurufen
fn get_tcp_connections() -> Result<Vec<(String, String, u32)>, String> {
    let mut buffer_size: u32 = 0;
    unsafe {
        // Ersten Aufruf machen, um die Größe des Puffers zu ermitteln
        GetExtendedTcpTable(
            ptr::null_mut(),
            &mut buffer_size,
            false.into(),
            2, // AF_INET (IPv4)
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );
    }

    // Speicher für die Tabelle allokieren
    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];
    unsafe {
        let result = GetExtendedTcpTable(
            buffer.as_mut_ptr() as *mut _,
            &mut buffer_size,
            false.into(),
            2,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if result.0 != 0 {
            return Err(format!("Failed to retrieve TCP table: {}", result.0));
        }

        // Tabelle analysieren
        let table = buffer.as_ptr() as *const MIB_TCPROW_OWNER_PID;
        let num_entries = *(table as *const u32) as usize;
        let rows = std::slice::from_raw_parts(table.offset(1), num_entries);

        Ok(rows
            .iter()
            .map(|row| {
                let local_address = format!(
                    "{}:{}",
                    u32_to_ipv4(row.dwLocalAddr),
                    u16::from_be(row.dwLocalPort as u16)
                );
                let remote_address = format!(
                    "{}:{}",
                    u32_to_ipv4(row.dwRemoteAddr),
                    u16::from_be(row.dwRemotePort as u16)
                );
                let pid = row.dwOwningPid;
                (local_address, remote_address, pid)
            })
            .collect())
    }
}

// Funktion, um UDP-Verbindungen abzurufen
fn get_udp_connections() -> Result<Vec<(String, u32)>, String> {
    let mut buffer_size: u32 = 0;
    unsafe {
        // Ersten Aufruf machen, um die Größe des Puffers zu ermitteln
        GetExtendedUdpTable(
            ptr::null_mut(),
            &mut buffer_size,
            false.into(),
            2, // AF_INET (IPv4)
            UDP_TABLE_OWNER_PID,
            0,
        );
    }

    // Speicher für die Tabelle allokieren
    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];
    unsafe {
        let result = GetExtendedUdpTable(
            buffer.as_mut_ptr() as *mut _,
            &mut buffer_size,
            false.into(),
            2,
            UDP_TABLE_OWNER_PID,
            0,
        );

        if result.0 != 0 {
            return Err(format!("Failed to retrieve UDP table: {}", result.0));
        }

        // Tabelle analysieren
        let table = buffer.as_ptr() as *const MIB_UDPROW_OWNER_PID;
        let num_entries = *(table as *const u32) as usize;
        let rows = std::slice::from_raw_parts(table.offset(1), num_entries);

        Ok(rows
            .iter()
            .map(|row| {
                let local_address = format!(
                    "{}:{}",
                    u32_to_ipv4(row.dwLocalAddr),
                    u16::from_be(row.dwLocalPort as u16)
                );
                let pid = row.dwOwningPid;
                (local_address, pid)
            })
            .collect())
    }
}

// Funktion zur Konvertierung einer IPv4-Adresse
fn u32_to_ipv4(addr: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        (addr & 0xFF),
        (addr >> 8) & 0xFF,
        (addr >> 16) & 0xFF,
        (addr >> 24) & 0xFF
    )
}