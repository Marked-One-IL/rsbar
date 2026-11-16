fn bytes_str(bytes: u64) -> String
{
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * KIB;
    const GIB: f64 = MIB * MIB;

    let bytes_f64 = bytes as f64;
    if bytes_f64 >= GIB {
        format!("{:.1}(GiB/s)", bytes_f64 / GIB)
    } else if bytes_f64 >= MIB {
        format!("{:.1}(MiB/s)", bytes_f64 / MIB)
    } else if bytes_f64 >= KIB {
        format!("{:.1}(KiB/s)", bytes_f64 / KIB)
    } else {
        format!("{}(B/s)", bytes)
    }
}

pub fn cpu() -> String
{
    let mut system = sysinfo::System::new().unwrap();
    system.refresh_cpu_usage();
    format!("cpu: {:.2}%", system.global_cpu_usage())
}
pub fn ram() -> String
{
    let mut system = sysinfo::System::new().unwrap();
    system.refresh_memory();
    let percentage = (system.used_memory() as f64 / system.total_memory() as f64) * 100.0;
    format!("ram: {:.2}%", percentage)
}
pub fn gpu() -> String
{
    let gpus = sysinfo::Gpus::new_with_refreshed_list().unwrap();
    let best_gpu = gpus.iter().max_by_key(|gpu| gpu.total_memory()).unwrap();
    let percentage = best_gpu.usage().unwrap();
    format!("gpu: {:.2}%", percentage)
}
pub fn vram() -> String {
    let gpus = sysinfo::Gpus::new_with_refreshed_list().unwrap();
    let best_gpu = gpus.iter().max_by_key(|gpu| gpu.total_memory()).unwrap();
    let percentage = (best_gpu.used_memory().unwrap() as f64 / best_gpu.total_memory().unwrap() as f64) * 100.0;
    format!("vram: {:.2}%", percentage)
}
pub fn disk() -> String
{
    let disks = sysinfo::Disks::new_with_refreshed_list().unwrap();
    let root = if cfg!(target_os = "windows") {"C:\\" } else { "/" };
    let os_disk = disks.iter().find(|disk| disk.mount_point() == root).unwrap();
    let percentage = (((os_disk.total_space() - os_disk.available_space()) as f64) / (os_disk.total_space() as f64)) * 100.0;
    format!("disk: {:.2}%", percentage)
}
pub fn network() -> String
{
    let mut networks = sysinfo::Networks::new_with_refreshed_list().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    networks.refresh(true);

    let mut total_up: u64 = 0;
    let mut total_down: u64 = 0;

    for (_, data) in networks.iter()
    {
        total_up += data.transmitted();
        total_down += data.received();
    }

    format!("up {} down {}", bytes_str(total_up), bytes_str(total_down))
}
pub fn time() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
