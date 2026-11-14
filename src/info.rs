use sysinfo::Gpu;

pub struct Inst {
    system: sysinfo::System
}
pub struct GpuInfo {
    pub usage_percentage: f32,
    pub vram_percentage: f32
}

impl Inst
{
    pub fn new() -> Self {
        Self { system: sysinfo::System::new().unwrap(),  }
    }

    pub fn get_cpu_percentage(&mut self) -> f32 {
        self.system.refresh_cpu_usage();
        self.system.global_cpu_usage()
    }
    pub fn get_ram_percentage(&mut self) -> f32 {
        self.system.refresh_memory();
        (self.system.used_memory() as f32 / self.system.total_memory() as f32) * 100.0
    }
    pub fn get_gpu_percentage(&mut self) -> GpuInfo
    {
        let gpus = sysinfo::Gpus::new_with_refreshed_list().unwrap();
        let gpu = &gpus[0];

        GpuInfo { usage_percentage: gpu.usage().unwrap(),
                  vram_percentage: (gpu.used_memory().unwrap() as f32 / gpu.total_memory().unwrap() as f32) * 100.0 }
    }
}