use sysinfo::System;

fn main() {
    let mut sys = System::new();
    sys.refresh_all();

    println!("=> system:");
    println!("System hostname: {:?}", System::host_name().unwrap());

}
