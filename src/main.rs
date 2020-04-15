use gpio_cdev::{Chip, LineRequestFlags};

fn main() {
    // Read the state of GPIO4 on a raspberry pi.  /dev/gpiochip0
// maps to the driver for the SoC (builtin) GPIO controller.
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let handle = chip
        .get_line(4).unwrap()
        .request(LineRequestFlags::INPUT, 0, "read-input").unwrap();
    for _ in 1..4 {
        println!("Value: {:?}", handle.get_value().unwrap());
    }

    println!("Hello, world!");
    println!("step 1");
    println!("step 2")
}
