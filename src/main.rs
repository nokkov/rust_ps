use std::{ffi::{OsStr, OsString}, io::{self, Write}, thread, time::Duration};
use chrono::{DateTime, Utc};
use sysinfo::{Process, ProcessStatus, System};
use termion::{clear, cursor};

//FIXME: &str or String?

const PROCESS_LIMIT: usize = 15;

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

fn main() {
    let mut sys = System::new();
    let mut stdout = io::stdout().lock();

    //TODO: alternate screen

    write!(stdout, "{}{}", cursor::Goto(1, 1), clear::All).unwrap();
    write!(stdout, "System hostname: {:?}\n", System::host_name().unwrap_or_else(|| "N/A".to_string())).unwrap();
    write!(stdout, "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10}",
    "PID", "NAME", "CPU%", "READ(B)", "WRITTEN(B)", "ELAPSED(MIN)", "STATUS", "CMD").unwrap();

    stdout.flush().unwrap();
    
    loop {

        write!(stdout, "{}{}", cursor::Goto(1, 5), clear::AfterCursor).unwrap(); 
        stdout.flush().unwrap();

        sys.refresh_all();

        for (pid, process) in sys.processes().iter().take(10) {
            let p_name = process.name().to_str().unwrap();
            let p_cpu_usage = process.cpu_usage();

            let p_disk_usage = process.disk_usage();

            let p_read_bytes = p_disk_usage.total_read_bytes;
            let p_written_bytes = p_disk_usage.total_written_bytes;
            
            let p_start_time_timestamp = process.start_time().to_string().parse::<i64>().unwrap();
            let p_start_time_date = DateTime::from_timestamp(p_start_time_timestamp, 0).unwrap();
            let current_date_time = Utc::now();
            let p_elapsed_time = (current_date_time - p_start_time_date).num_minutes();
            

            let p_status = process.status();

            let p_cmd_os_string= process.cmd().join(&OsString::from(" "));
            let p_cmd = p_cmd_os_string.to_str().expect("Error during converting osstr -> str");

            println!(
                "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10}", 
                truncate_and_ellipsis(&pid.to_string(), 10), 
                truncate_and_ellipsis(&p_name.to_string(), 10),
                truncate_and_ellipsis(&p_cpu_usage.to_string(), 10), 
                truncate_and_ellipsis(&p_read_bytes.to_string(), 10),
                truncate_and_ellipsis(&p_written_bytes.to_string(), 10), 
                truncate_and_ellipsis(&p_elapsed_time.to_string(), 10),
                truncate_and_ellipsis(&p_status.to_string(), 10), 
                truncate_and_ellipsis(&p_cmd.to_string(), 10)
            )
        }

        thread::sleep(Duration::from_secs(5)); // TODO: custom refresh time
    }
}
