use std::ffi::OsString;

use sysinfo::System;

fn main() {
    let mut sys = System::new();
    sys.refresh_all();

    // println!("=> system:");
    // println!("System hostname: {:?}", System::host_name().unwrap());

    println!("[pid] [name] [cpu_usage] [read_bytes] [written_bytes] [start_time] [status] [cmd]");

    for (pid, process) in sys.processes() {
        let p_name = process.name().to_str().unwrap(); // add checks
        let p_cpu_usage = process.cpu_usage();

        let p_disk_usage = process.disk_usage();
        let p_read_bytes = p_disk_usage.total_read_bytes;
        let p_written_bytes = p_disk_usage.total_written_bytes;

        let p_start_time = process.start_time(); // convert
        let p_status = process.status();

        let p_cmd_os_string= process.cmd().join(&OsString::from(" "));
        let p_cmd = p_cmd_os_string.to_str().expect("Error during converting osstr -> str");

        println!(
            "{0} {1} {2} {3} {4} {5} {6} {7}", 
            pid, 
            p_name, 
            p_cpu_usage, 
            p_read_bytes,
            p_written_bytes, 
            p_start_time,
            p_status, 
            p_cmd
        )
    }
}
