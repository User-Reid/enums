#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOs,
    Linux,
}

fn years_since_release(os: &OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => 39,
        OperatingSystem::MacOs => 23,
        OperatingSystem::Linux => 23,
    }
}

fn main() {
    let my_computer: OperatingSystem = OperatingSystem::Windows;
    println!(
        "My {:?} operating system was released {:?} years ago!",
        my_computer,
        years_since_release(&my_computer)
    )
}
