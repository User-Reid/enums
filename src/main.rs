#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOs,
    Linux,
}

fn years_since_release(os: &OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite a pretty damn old computer!");
            39
        }
        OperatingSystem::MacOs => 23,
        OperatingSystem::Linux => 23,
    }
}

fn main() {
    let my_computer: OperatingSystem = OperatingSystem::Windows;
    let age: u32 = years_since_release(&my_computer);
    println!("My dad's computer is {age}")
}
