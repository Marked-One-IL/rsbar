pub fn cpu() -> String {
    let mut system = sysinfo::System::new().unwrap();
    system.refresh_cpu_usage();
    format!("cpu: {:.2}%", system.global_cpu_usage())
}
pub fn ram() -> String {
    let mut system = sysinfo::System::new().unwrap();
    system.refresh_memory();
    format!("ram: {:.2}%", (system.used_memory() as f32 / system.total_memory() as f32) * 100.0)
}
pub fn gpu() -> String
{
    let gpus = sysinfo::Gpus::new_with_refreshed_list().unwrap();
    let best_gpu = gpus.iter().max_by_key(|gpu| gpu.total_memory()).unwrap();
    format!("gpu: {:.2}%", best_gpu.usage().unwrap())
}
pub fn vram() -> String
{
    let gpus = sysinfo::Gpus::new_with_refreshed_list().unwrap();
    let best_gpu = gpus.iter().max_by_key(|gpu| gpu.total_memory()).unwrap();
    format!("vram: {:.2}%", (best_gpu.used_memory().unwrap() as f32 / best_gpu.total_memory().unwrap() as f32) * 100.0)
}
pub fn time() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}