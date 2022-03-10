use evdev_rs::enums::{EventType, EV_KEY, EV_SYN, EventCode};
use evdev_rs::uinput::UInputDevice;
use evdev_rs::{TimeVal, UninitDevice, InputEvent, DeviceWrapper};

use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn main() {
    gen_event(Duration::from_secs(1));
}

/// Create an uinput virtual device
/// and continously emits events from it.
fn gen_event(loop_delay: Duration) {

    fn build_timeval() -> TimeVal {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        TimeVal::new(now.as_secs().try_into().unwrap(), now.as_micros().try_into().unwrap())
    }

    let mut dev = UninitDevice::new().unwrap();
    dev.set_name(&"virtual device");
    dev.enable(&EventType::EV_KEY).unwrap();
    dev.enable(&EventCode::EV_KEY(EV_KEY::KEY_SPACE)).unwrap();

    let uinput_dev = UInputDevice::create_from_device(&dev).unwrap();

    // await device creation
    sleep(Duration::from_secs(1));

    eprintln!("Created uinput device: {}", uinput_dev.devnode().unwrap());

    loop {
        let timeval = build_timeval();
        let press = InputEvent::new(&timeval, &EventCode::EV_KEY(EV_KEY::KEY_SPACE), 1);
        let report = InputEvent::new(&timeval, &EventCode::EV_SYN(EV_SYN::SYN_REPORT), 0);
        uinput_dev.write_event(&press).unwrap();
        uinput_dev.write_event(&report).unwrap();
        eprintln!("Pressed key");
        sleep(loop_delay);

        let timeval = build_timeval();
        let release = InputEvent::new(&timeval, &EventCode::EV_KEY(EV_KEY::KEY_SPACE), 0);
        let report = InputEvent::new(&timeval, &EventCode::EV_SYN(EV_SYN::SYN_REPORT), 0);
        uinput_dev.write_event(&release).unwrap();
        uinput_dev.write_event(&report).unwrap();
        eprintln!("Released key");
        sleep(loop_delay);
    }
}
