use std::{error::Error, ffi::{OsString}, io::{self, stdout, Write}, thread, time::Duration};
use chrono::{DateTime, Utc};
use sysinfo::{Process, ProcessStatus, System};
use termion::{clear, cursor, screen::IntoAlternateScreen};
use clap::{Parser, ValueEnum};

const REFRESH_DURATION: Duration = Duration::from_secs(2);
const DEFAULT_PROCESS_LIMIT: usize = 15;
const COL_WIDTH: usize = 12;

#[derive(Parser, Debug)]
#[command(author, version, about = "simple ps written on rust", long_about = None)]
struct Cli {
    #[arg(short, long, value_enum, default_value_t = SortBy::Cpu)]
    sort_by: SortBy,

    #[arg(short, long, default_value_t = DEFAULT_PROCESS_LIMIT as u32)]
    limit: u32,

    #[arg(short, long)]
    watch: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum SortBy {
    Cpu,
    Pid,
    Name,
    Uptime,
    Status,
    ReadBytes,
    WrittenBytes,
}

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
    if max_len <= 3 {
        return ".".repeat(max_len);
    }
    if s.len() > max_len {
        format!("{:.width$}...", s, width = max_len - 3)
    } else {
        s.to_string()
    }
}

fn gather_process_data(sys: &System, sort_by: SortBy, limit: usize) -> Vec<ProcessInfo> {
    let mut processes: Vec<&Process> = sys.processes().values().collect();

    match sort_by {
        SortBy::Cpu => processes.sort_unstable_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal)),
        SortBy::Pid => processes.sort_unstable_by_key(|p| p.pid().as_u32()),
        SortBy::Name => processes.sort_unstable_by_key(|p| p.name().to_ascii_lowercase()),
        SortBy::Uptime => processes.sort_unstable_by_key(|p| p.start_time()),
        SortBy::Status => processes.sort_unstable_by_key(|p| p.status()),
        SortBy::ReadBytes => processes.sort_unstable_by_key(|p| p.disk_usage().read_bytes),
        SortBy::WrittenBytes => processes.sort_unstable_by_key(|p| p.disk_usage().written_bytes),
    }

    processes
        .iter()
        .take(limit)
        .map(|process| {
            let p_disk_usage = process.disk_usage();
            
            let p_start_time_date = DateTime::from_timestamp_millis(process.start_time() as i64)
                                        .unwrap_or_else(|| Utc::now());
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
                cmd: OsString::from(process.cmd().join(&OsString::from(" "))), 
}
        })
        .collect()
}

fn draw_ui<W: Write>(screen: &mut W, processes: &[ProcessInfo]) -> io::Result<()> {
    write!(screen, "{}{}", cursor::Goto(1, 1), clear::All)?;

    writeln!(screen, "System Hostname: {}\r", System::host_name().unwrap_or_else(|| "N/A".to_string()))?;

    writeln!(screen, "{0: <width$} | {1: <width$} | {2: <width$} | {3: <width$} | {4: <width$} | {5: <width$} | {6: <width$} | {7: <width_cmd$}\r",
        "PID", "NAME", "CPU %", "READ(B)", "WRITTEN(B)", "UPTIME(m)", "STATUS", "COMMAND", width = COL_WIDTH, width_cmd = COL_WIDTH * 2)?;
    writeln!(screen, "{}\r", "─".repeat(COL_WIDTH * 7 + (COL_WIDTH * 2) + 7 * 3))?;

    for p_info in processes {
        let name_str = p_info.name.to_string_lossy();
        let cmd_str = p_info.cmd.to_string_lossy();

        writeln!(
            screen,
            "{0: <width$} | {1: <width$} | {2: <width$.2} | {3: <width$} | {4: <width$} | {5: <width$} | {6: <width$} | {7: <width_cmd$}\r",
            truncate_and_ellipsis(&p_info.pid.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&name_str, COL_WIDTH),
            p_info.cpu_usage,
            truncate_and_ellipsis(&p_info.read_bytes.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.written_bytes.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.elapsed_time.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&p_info.status.to_string(), COL_WIDTH),
            truncate_and_ellipsis(&cmd_str, COL_WIDTH * 2),
            width = COL_WIDTH,
            width_cmd = COL_WIDTH * 2
        )?;
    }

    screen.flush()
}

fn run_tui<W: Write>(screen: &mut W, sys: &mut System, sort_by: SortBy, limit: usize) -> Result<(), Box<dyn Error>> {
    loop {
        sys.refresh_all();
        let processes = gather_process_data(sys, sort_by, limit);
        draw_ui(screen, &processes)?;
        thread::sleep(REFRESH_DURATION);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut sys = System::new_all();

    if cli.watch {
        let mut screen = stdout().into_alternate_screen()?;
        if let Err(e) = run_tui(&mut screen, &mut sys, cli.sort_by, cli.limit as usize) {
            eprintln!("Application error: {}", e);
            return Err(e);
        }
    } else {
        sys.refresh_all();
        let processes = gather_process_data(&sys, cli.sort_by, cli.limit as usize);
        let mut stdout_handle = stdout();
        draw_ui(&mut stdout_handle, &processes)?;
    }
    
    Ok(())
}