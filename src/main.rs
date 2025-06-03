use sysinfo::System;

fn main() {
    let mut sys = System::new();
    sys.refresh_all();

    // println!("=> system:");
    // println!("System hostname: {:?}", System::host_name().unwrap());

    println!("[pid] [name] [cpu_usage] [disk_usage] [start_time] [status] [cmd]");

    for (pid, process) in sys.processes() {
        let p_name = process.name().to_str().unwrap(); // add checks
        let p_cpu_usage = process.cpu_usage();
        let p_disk_usage = process.disk_usage().total_written_bytes; // fixme
        let p_start_time = process.start_time(); // convert
        let p_status = process.status();
        let p_cmd = process.cmd();
        println!(
            "{0} {1} {2} {3} {4} {5}", 
            pid, 
            p_name, 
            p_cpu_usage, 
            p_disk_usage, 
            p_start_time, 
            p_status, 
            // p_cmd
        )
    }
}
