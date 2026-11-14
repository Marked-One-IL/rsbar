mod info;

fn main()
{
    let mut info = info::Inst::new();

    println!("Cpu: {}%", info.get_cpu_percentage());
    println!("Ram: {}%", info.get_ram_percentage());

    let gpu = info.get_gpu_percentage();
    println!("Gpu usage: {}%", gpu.usage_percentage);
    println!("Gpu vram: {}%", gpu.vram_percentage);
}