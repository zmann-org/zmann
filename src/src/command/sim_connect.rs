use std::{io::BufReader, thread, time::Duration};

use crate::command::events::{AircraftData, SimData};
use log::info;
use simconnect_sdk::SimConnect as Sim;

pub struct SimConnect {}

impl SimConnect {
    pub fn new() -> Self {
        thread::spawn(|| {
            let client = Sim::new("VirtualFlyer");
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&handle).unwrap();
            let file = std::fs::File::open("C:/Users/zmann/Downloads/audio/pax.mp3").unwrap();
            sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
            
            let sink2 = rodio::Sink::try_new(&handle).unwrap();
            let file2 = std::fs::File::open("C:/Users/zmann/Downloads/audio/screaming_passengers.wav").unwrap();
            sink2.append(rodio::Decoder::new(BufReader::new(file2)).unwrap());
            sink.play();
            match client {
                Ok(mut c) => loop {
                    {
                        match c.get_next_dispatch() {
                            Ok(notification) => match notification {
                                Some(event) => match event {
                                    simconnect_sdk::Notification::Open => {
                                        println!("SIM CONNECTED");
                                        c.register_object::<SimData>().ok();
                                        c.register_object::<AircraftData>().ok();
                                    }
                                    simconnect_sdk::Notification::Object(obj) => {
                                        if let Ok(sim_data) = SimData::try_from(&obj) {
                                            info!("{:?}", sim_data);
                                        }
                                        if let Ok(sim_data) = AircraftData::try_from(&obj) {
                                            info!("{:?}, {:?}", sim_data.view, sim_data.g_force);
                                            if (sim_data.view == 2.0) {
                                                if sim_data.g_force >= -1.0
                                                    && sim_data.g_force <= 2.5
                                                {
                                                    sink2.pause();
                                                    sink.play();
                                                } else {
                                                    sink.pause();
                                                    sink2.play();
                                                }
                                            } else {
                                                sink.pause();
                                                sink2.pause();
                                            }
                                        }
                                    }
                                    _ => {}
                                },
                                None => {}
                            },
                            Err(_) => {}
                        }
                    }
                    thread::sleep(Duration::from_millis(20));
                },
                Err(_) => {}
            }
        });
        Self {}
    }
}
