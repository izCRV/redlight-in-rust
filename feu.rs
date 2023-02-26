use std::thread;
use std::time::Duration;

fn main() {
    loop {
        print_circle("🟢");
        thread::sleep(Duration::from_secs(10));

        print_circle("🟠");
        thread::sleep(Duration::from_secs(1));

        print_circle("🔴");
        thread::sleep(Duration::from_secs(10));
    }
}

fn print_circle(circle: &str) {
    println!("{}", circle);
}

