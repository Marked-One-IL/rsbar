mod info;

fn main() {
    let mut info = info::Inst::new();
    println!("{}", info.get_cpu_data());
}