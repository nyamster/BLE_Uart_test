use btleplug::api::UUID::B16;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};

use std::convert::TryInto;
#[allow(dead_code)]
#[allow(unused_imports)]
use std::thread;

#[allow(unused_imports)]
use btleplug::api::{Central, Characteristic, Peripheral, UUID};
#[allow(unused_imports)]
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::Adapter, adapter::ConnectedAdapter, manager::Manager};

#[cfg(target_os = "linux")]
fn connect_to(adapter: &Adapter) -> ConnectedAdapter {
    adapter
        .connect()
        .expect("Error connecting to BLE Adapter....") //linux
}
#[cfg(target_os = "linux")]
fn print_adapter_info(adapter: &ConnectedAdapter) {
    println!(
        "connected adapter {:?} is UP: {:?}",
        adapter.adapter.name,
        adapter.adapter.is_up()
    );
    println!("adapter states : {:?}", adapter.adapter.states);
}

use serialport::{DataBits, FlowControl, SerialPort, StopBits};
use std::io::{self};
use std::time::{Duration, Instant};
use std::fmt::Write;

use std::fs::File;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::Path;

fn bluetooth_connect() -> Option<impl Peripheral> {
    let manager = Manager::new().unwrap();
    let adapter_list = manager.adapters().unwrap();

    if adapter_list.len() <= 0 {
        eprint!("Bluetooth adapter(s) were NOT found, sorry...\n");
        return None;
    } else {
        //for adapter in adapter_list.iter() {
        let adapter = &adapter_list[0];
        println!("connecting to BLE adapter: ...");

        let connected_adapter = if cfg!(windows) {
            connect_to(&adapter)
        } else {
            connect_to(&adapter)
        };
        // let connected_adapter = connect_to(&adapter);
        print_adapter_info(&connected_adapter);
        connected_adapter
            .start_scan()
            .expect("Can't scan BLE adapter for connected devices...");
        thread::sleep(Duration::from_secs(2));
        if connected_adapter.peripherals().is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
            return None;
        } else {
            // all peripheral devices in range
            for peripheral in connected_adapter.peripherals().into_iter() {
                println!(
                    "peripheral : {:?} is connected: {:?}",
                    peripheral.properties().local_name,
                    peripheral.is_connected()
                );
                if peripheral.properties().local_name.is_some()
                    && peripheral.properties().local_name.unwrap() == "Feasycom"
                    && !peripheral.is_connected()
                {
                    println!(
                        "start connect to peripheral : {:?}...",
                        peripheral.properties().local_name
                    );
                    peripheral
                        .connect()
                        .expect("Can't connect to peripheral...");
                    println!(
                        "now connected (\'{:?}\') to peripheral : {:?}...",
                        peripheral.is_connected(),
                        peripheral.properties().local_name
                    );
                    let chars = peripheral.discover_characteristics().unwrap();

                    if peripheral.is_connected() {
                        println!(
                            "Discover peripheral : \'{:?}\' characteristics...",
                            peripheral.properties().local_name
                        );
                        //let write = chars.iter().find(|c| c.uuid == B16(0xFFF2)).unwrap();
                        let notify = chars.iter().find(|c| c.uuid == B16(0xFFF1)).unwrap();
                        let write = chars.iter().find(|c| c.uuid == B16(0xFFF2)).unwrap();

                        //peripheral.command(write, b"1").unwrap();
                        peripheral.subscribe(notify).unwrap();
                        return Some(peripheral);
                    }
                    break;
                } else {
                    //sometimes peripheral is not discovered completely
                    eprintln!("SKIP connect to UNKNOWN peripheral : {:?}", peripheral);
                }
            }
        }
    }
    return None;
}

fn write_command(port: &mut Box<dyn SerialPort>, command: &str) -> Option<String> {
    let _ = port.write(command.as_bytes()).expect("Write failed!");
    let mut input  = [0u8; 80];
    loop {
        match port.read(&mut input) {
            Ok(n) => {
                let ans = std::str::from_utf8(&mut input[..n]);
                if let Ok(ans) = ans {
                    return Some(ans.to_string());
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {eprintln!("{:?}", e); return None},
        }
    }
}

fn write_byte_command(port: &mut Box<dyn SerialPort>, command: &[u8]) -> Option<String> {
    let _ = port.write(command).expect("Write failed!");
    let mut input  = [0u8; 80];
    loop {
        match port.read(&mut input) {
            Ok(n) => {
                let ans = std::str::from_utf8(&mut input[..n]);
                if let Ok(ans) = ans {
                    return Some(ans.to_string());
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {eprintln!("{:?}", e); return None},
        }
    }
}

fn write_sin() {
    let path = Path::new("data.bin");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
}

fn main() {
    let mut sin = [0i32; 20];
    
    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .flow_control(FlowControl::None)
        .stop_bits(StopBits::One)
        .data_bits(DataBits::Eight)
        .open()
        .expect("Failed to open port");

    let mut input  = [0u8; 80];
    // loop {
    //     match port.read(&mut input) {
    //         Ok(n) => {
    //             let ans = std::str::from_utf8(&mut input[..n]);
    //             if let Ok(ans) = ans {
    //                 println!("{}", ans.to_string());
    //             }
    //         }
    //         Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
    //         Err(e) => {eprintln!("{:?}", e)},
    //     }
    // }


    // let res = write_command(&mut port, "AT+RESTORE\r\n");
    // println!("Answer: {}", res.unwrap());
    
    // let res = write_command(&mut port, "AT+TPMODE=0\r\n");
    // println!("Answer: {}", res.unwrap());
    // let res = write_command(&mut port, "AT+LPM=1\r\n");
    // println!("Answer: {}", res.unwrap());
    // let res = write_command(&mut port, "AT+PIOCFG=1,1\r\n");
    // println!("Answer: {}", res.unwrap());
    
    // let res = write_command(&mut port, "AT+LESEND=30,012345678901234567890123456789\r\n");
    // println!("Answer: {}", res.unwrap());

    loop {
        match port.read(&mut input) {
            Ok(n) => {
                let ans = std::str::from_utf8(&mut input[..n]);
                if let Ok(ans) = ans {
                    println!("{}", ans.to_string());
                    if ans.to_string() == "+GATTDATA=5,AdcOn\r\n" {
                        println!("{}", ans.to_string());
                        break;
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {eprintln!("{:?}", e)},
        }
    }

    for i in 0..20 {
        sin[i%20] = ((i as f64 / 50.0).sin() * 100.0) as i32;

        if i % 20 == 19 {
            // let mut vec: Vec<u8> = Vec::new();
            // vec.extend_from_slice("AT+LESEND=20,".as_bytes());
            // let v_bytes: &[u8] = unsafe {
            //     std::slice::from_raw_parts(
            //         sin.as_ptr() as *const u8,
            //         sin.len() * std::mem::size_of::<i32>(),
            //     )
            // };
            // vec.extend_from_slice(&v_bytes);
            // vec.extend_from_slice("\r\n".as_bytes());
            let mut buf = String::new();
            let mut msg = String::new();
            let _ = write!(&mut msg, "{:?}", &sin[0..20]);
            let _ = write!(&mut buf, "AT+LESEND={},{}\r\n", msg.len(), msg);
            println!("MESSAGE: {:?}", buf);
            let res = write_command(&mut port, &buf);
            //let res = write_byte_command(&mut port, &vec);
            // // if res.unwrap() == "AdcOff" {
            // //     break;
            // // }
            println!("Answer: {}", res.unwrap());
        }
    }
    loop{}


    let peripheral = bluetooth_connect().unwrap();
    let mut t1 = Instant::now();
    let mut cnt = 0;
    let mut i = 0;
    peripheral.on_notification(Box::new(move |v| {
        //cnt += v.value.len();
        println!("Data: {:x?}", &v.value);
        // for x in &v.value {
        //     if *x != i {
        //         println!("Data: {:x?}", &v.value);
        //         assert_eq!(*x, i);
        //         println!("ERROR");
        //     }
        //     if i == 31 {
        //         i = 0;
        //     } else {
        //         i += 1;
        //     }
        // }

        //println!("Cnt: {}", &data_cnt);
        // let t2 = Instant::now();
        // if t2.duration_since(t1) > Duration::from_secs(1) {
        //     t1 = t2;
        //     println!("Byte per sec: {}", cnt);
        //     cnt = 0;
        // }
    }));
}
