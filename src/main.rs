mod info;

fn main()
{
    println!("{}", info::cpu());
    println!("{}", info::ram());
    println!("{}", info::gpu());
    println!("{}", info::vram());
    println!("{}", info::time());
}