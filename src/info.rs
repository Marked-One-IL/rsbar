pub struct Inst {
    system: sysinfo::System
}

impl Inst
{
    pub fn new() -> Self {
        Self { system: sysinfo::System::new().unwrap(),  }
    }

    pub fn get_cpu_data(&mut self) -> String {
        self.system.refresh_cpu_usage();
        format!("Cpu: {}%", self.system.global_cpu_usage())
    }
    pub fn get_ram_data(&mut self) -> String {
        self.system.refresh_memory();
        format!("Ram: {}%", (self.system.used_memory() as f32 / self.system.total_memory() as f32) * 100.0)
    }
    pub fn get_gpu_data(&mut self) -> String
    {
        let gpus = sysinfo::Gpus::new_with_refreshed_list().unwrap();
        let gpu = &gpus[0];

        format!(concat!("Gpu usage: {}%\n",
                        "Gpu vram: {}"),
                        gpu.usage().unwrap(),
                        (gpu.used_memory().unwrap() as f32 / gpu.total_memory().unwrap() as f32) * 100.0)
    }
}