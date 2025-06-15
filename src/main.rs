use std::{error::Error, ffi::{OsString}, io::{self, stdout, Write}, thread, time::Duration};
use chrono::{DateTime, Utc};
use sysinfo::{Process, ProcessStatus, System};
use termion::{clear, cursor, screen::IntoAlternateScreen};

const REFRESH_DURATION: Duration = Duration::from_secs(2);
const PROCESS_LIMIT: usize = 15;
const COL_WIDTH: usize = 12;

struct ProcessInfo {
    pid: u32,
    name: OsString,
    cpu_usage: f32,
    read_bytes: u64,
    written_bytes: u64,
    elapsed_time: i64, 
    status: ProcessStatus,
    cmd: OsString
}

fn truncate_and_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{:.width$}...", s, width = max_len - 3)
    } else {
        s.to_string()
    }
}

fn gather_process_data(sys: &System) -> Vec<ProcessInfo> {
    let mut processes: Vec< &Process> = sys.processes().values().collect();
    processes.sort_unstable_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));

    processes
        .iter()
        .take(PROCESS_LIMIT)
            .map(|process| {
                let p_disk_usage = process.disk_usage();
                
                let p_start_time_timestamp = process.start_time().to_string().parse::<i64>().unwrap();
                let p_start_time_date = DateTime::from_timestamp(p_start_time_timestamp, 0).unwrap();
                let current_date_time = Utc::now();
                let p_elapsed_time = (current_date_time - p_start_time_date).num_minutes();

                ProcessInfo {
                    pid: process.pid().as_u32(),
                    name: process.name().to_os_string(),
                    cpu_usage: process.cpu_usage(),
                    read_bytes: p_disk_usage.read_bytes,
                    written_bytes: p_disk_usage.written_bytes,
                    elapsed_time: p_elapsed_time,
                    status: process.status(),
                    cmd: process.cmd().join(&OsString::from(" ")),
                }
            })
            .collect()
}

fn draw_ui<W: Write>(screen: &mut W, processes: &[ProcessInfo]) -> io::Result<()> {
    write!(screen, "{}{}", cursor::Goto(1, 1), clear::All)?;

    writeln!(screen, "System Hostname: {}\r", System::host_name().unwrap_or_else(|| "N/A".to_string()))?;

    writeln!(screen, "{0: <width$} | {1: <width$} | {2: <width$} | {3: <width$} | {4: <width$} | {5: <width$} | {6: <width$} | {7: <width$}\r",
        "PID", "NAME", "CPU %", "READ(B)", "WRITTEN(B)", "UPTIME(m)", "STATUS", "COMMAND", width = COL_WIDTH)?;
    writeln!(screen, "{}\r", "─".repeat(COL_WIDTH * 8 + 7 * 3))?;

    for p_info in processes {
        
        writeln!(
            screen,
            "{0: <width$} | {1: <width$} | {2: <width$} | {3: <width$} | {4: <width$} | {5: <width$} | {6: <width$} | {7: <width$}\r",
            truncate_and_ellipsis(&p_info.pid.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.name.to_os_string().into_string().unwrap(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.cpu_usage.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.read_bytes.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.written_bytes.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.elapsed_time.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.status.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.cmd.to_os_string().into_string().unwrap(), COL_WIDTH * 2),
            width = COL_WIDTH
        )?;
    }

    screen.flush()
}

fn run<W: Write>(screen: &mut W, sys: &mut System) -> Result<(), Box<dyn Error>> {
    loop {
        sys.refresh_all();
        let processes = gather_process_data(sys);
        draw_ui(screen, &processes)?;
        thread::sleep(REFRESH_DURATION);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sys = System::new_all();
    let mut screen = stdout().into_alternate_screen()?;
    
    if let Err(e) = run(&mut screen, &mut sys) {
        eprintln!("Application error: {}", e);
        return Err(e);
    }
    
    Ok(())
}