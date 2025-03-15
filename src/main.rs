mod hardware;

fn main() {
    let device = hardware::Hardware::new();
    device.test_hardware();
}
