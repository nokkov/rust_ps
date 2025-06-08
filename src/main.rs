use std::ffi::OsString;
use chrono::{DateTime, Utc};
use sysinfo::System;

fn truncate_and_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{:.width$}...", s, width = max_len - 3)
    } else {
        s.to_string()
    }
}

fn main() {
    let mut sys = System::new();
    sys.refresh_all();

    // println!("=> system:");
    // println!("System hostname: {:?}", System::host_name().unwrap());

    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10}",
        "PID", "NAME", "CPU%", "READ(B)", "WRITTEN(B)", "ELAPSED(M)", "STATUS", "CMD"
    );

    for (pid, process) in sys.processes() {
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
}
