use std::ffi::OsString;
use chrono::{DateTime, Utc};
use sysinfo::System;

fn main() {
    let mut sys = System::new();
    sys.refresh_all();

    // println!("=> system:");
    // println!("System hostname: {:?}", System::host_name().unwrap());

    println!("[pid] [name] [cpu_usage] [read_bytes] [written_bytes] [elapsed_time] [status] [cmd]");

    for (pid, process) in sys.processes() {
        let p_name = process.name().to_str().unwrap();
        let p_cpu_usage = process.cpu_usage();

        let p_disk_usage = process.disk_usage();
        let p_read_bytes = p_disk_usage.total_read_bytes;
        let p_written_bytes = p_disk_usage.total_written_bytes;

        let p_start_time_timestamp = process.start_time().to_string().parse::<i64>().unwrap();
        let p_start_time_date = DateTime::from_timestamp(p_start_time_timestamp, 0).unwrap();
        let current_date_time = Utc::now();
        let p_elapsed_time = current_date_time - p_start_time_date;

        let p_status = process.status();

        let p_cmd_os_string= process.cmd().join(&OsString::from(" "));
        let p_cmd = p_cmd_os_string.to_str().expect("Error during converting osstr -> str");

        println!(
            "{0: <20} | {1: <20} | {2: <20} | {3: <20} | {4: <20} | {5: <20} | {6: <20} | {7: <20}", 
            pid, 
            p_name, 
            p_cpu_usage, 
            p_read_bytes,
            p_written_bytes, 
            p_elapsed_time,
            p_status, 
            p_cmd
        )
    }
}
