#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::str;
#[allow(dead_code)]
#[allow(unused_imports)]
use std::thread;
use btleplug::api::UUID::B16;

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

use serialport::{DataBits, FlowControl, StopBits};
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn bluetooth_connect() -> Option<impl Peripheral> {
    let manager = Manager::new().unwrap();
    let adapter_list = manager.adapters().unwrap();
    
    if adapter_list.len() <= 0 {
        eprint!("Bluetooth adapter(s) were NOT found, sorry...\n");
        return None
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
            return None
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

                        peripheral.subscribe(notify).unwrap();
                        return Some(peripheral)
                        // let mut data_cnt = 0;
                        
                        

                        // //peripheral.read(notify).unwrap();
                        // println!("1");

                        // peripheral.on_notification(Box::new(move |v| {
                        //     cnt += v.value.len();
                        //     data_cnt += 32;
                        //     //println!("Cnt: {}", &data_cnt);
                        //     let t2 = Instant::now();
                        //     if t2.duration_since(t1) > Duration::from_secs(1)
                        //     {
                        //         t1 = t2;
                        //         println!("Byte per sec: {}", cnt);
                        //         cnt = 0;
                        //     }
                        // }));

                        // loop {
                        // }
                    }
                    break;
                } else {
                    //sometimes peripheral is not discovered completely
                    eprintln!("SKIP connect to UNKNOWN peripheral : {:?}", peripheral);
                    return None
                }
            }
        }
    }
    return None
}

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 921600)
        .flow_control(FlowControl::None)
        .stop_bits(StopBits::One)
        .data_bits(DataBits::Eight)
        .open()
        .expect("Failed to open port");

    let now = Instant::now();
    const PACKETS_NUM: usize = 10;
    //let _ = port.write("AT+FLOWCTL\r\n".as_bytes()).expect("Write failed!");
    let mut input  = [0u8; 80];

    let peripheral = bluetooth_connect().unwrap();
    let mut t1 = Instant::now();
    let mut cnt = 0;
    let mut i = 0;
    peripheral.on_notification(Box::new(move |v| {
        cnt += v.value.len();
        //println!("Data: {:x?}", &v.value);
        for x in &v.value {
            if *x != i {
                assert_eq!(*x, i);
                println!("ERROR");
            }
            if i == 32 {
                i = 0;
            } else {
                i += 1;
            }
        }

        //println!("Cnt: {}", &data_cnt);
        let t2 = Instant::now();
        if t2.duration_since(t1) > Duration::from_secs(1)
        {
            t1 = t2;
            println!("Byte per sec: {}", cnt);
            cnt = 0;
        }
    }));
    // loop {
    //     match port.read(&mut input) {
    //         Ok(n) => {
    //             let ans = std::str::from_utf8(&mut input[..n]);
    //             if let Ok(ans) = ans {
    //                 println!("{}", ans);
    //             }
    //         }
    //         Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
    //         Err(e) => eprintln!("{:?}", e),
    //     }
    // }
    for j in 0..PACKETS_NUM {
        for i in 0..32 {
            let mut output = [i as u8; 32];
            for k in 0..32 {
                output[k] = ((i * j + k) % 256) as u8;
            }
            //println!("Data: {:x?}", &output);
            let _ = port.write(&output).expect("Write failed!");
            //println!("Returnal: {}", ret);
            std::thread::sleep(Duration::from_nanos((1000000.0 * 1000.0 / (1200 as f32)) as u64));
        }
    }
}
