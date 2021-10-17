use std::sync::Arc;
use std::net::UdpSocket;
use std::os::unix::net::UnixListener;
use std::io::prelude::*;
use std::fs;
use std::path::Path;
use std::thread;
use std::env;

use anyhow::Result;
use serde;
use serde::{Serialize, Deserialize};
use serde_json;
use stunclient::StunClient;
use local_ip_address::local_ip;

use p2p_audio::udp::client::{UdpClient};
use p2p_audio::audio::{AudioInterface, AudioConfig};
use p2p_audio::util::Mode;
use p2p_audio::ringbuffer;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum RecvMessage {
    #[serde(rename = "config")]
    Config,
    #[serde(rename = "connect")]
    Connect {
        config: Option<AudioConfig>
    },
    #[serde(rename = "stream")]
    Stream {
        mode: Mode,
        remote_addr: String,
        config: AudioConfig
    },
}


#[derive(Serialize, Debug)]
#[serde(tag = "type")]
enum SendMessage {
    #[serde(rename = "connect")]
    Connect { address: String, is_valid: bool },
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let socket = match args.get(1) {
        Some(path) => path,
        None => panic!("No socket path")
    };

    let path = Path::new(&socket);
    if path.exists() == true {
        fs::remove_file(path).expect("Could not delete socket");
    }

    let listener = UnixListener::bind(path)?;
    let conn = UdpSocket::bind("0:0")?;

    let conn = Arc::new(conn);

    // Iterate over clients, blocks if no client available
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        loop {
            let mut buf = [0u8; 256];
            let read = stream.read(&mut buf).unwrap();
            if read < 256 {
                continue;
            }
            let msg = std::str::from_utf8(&buf).unwrap();
            let msg = msg.trim_matches(char::from(0));

            let res: RecvMessage = serde_json::from_str(msg)?;

            match res {
                RecvMessage::Config => {
                    let supported_configs = AudioInterface::get_supported_configs().unwrap();
                    let res = serde_json::to_string(&supported_configs).unwrap();
                    stream.write_all(res.as_bytes());
                },
                RecvMessage::Connect { config } => {
                    let config = match config {
                        Some(config) => config,
                        None => AudioConfig::default()
                    };

                    let is_valid = match AudioInterface::validate_config(&config) {
                        Ok(_) => true,
                        Err(_) => false
                    };

                    let sc = StunClient::with_google_stun_server();
                    let addr = sc.query_external_address(&conn)?;
                    // let addr = conn.local_addr().unwrap();
                    // let ip = local_ip().unwrap();
                    // let addr = format!("{}:{}", ip, addr.port());
                    let res = SendMessage::Connect { address: addr.to_string(), is_valid };
                    let res = serde_json::to_string(&res).unwrap();

                    stream.write_all(res.as_bytes());
                },
                RecvMessage::Stream { mode, remote_addr, config } => {
                    let conn = conn.clone();
                    conn.connect(&remote_addr)?;
                    run_stream(mode, conn, config);
                }
            }
        }
    }

    Ok(())
}

fn run_stream(mode: Mode, conn: Arc<UdpSocket>, audio_config: AudioConfig) -> Result<()> {
    let audio_interface = AudioInterface::new(audio_config.clone())?;

    let packet_buffer_size = (audio_config.buffer_size * audio_config.get_channel_count() as u32) as usize;
    let (input_producer, input_consumer, output_producer, output_consumer) = ringbuffer::create(packet_buffer_size);

    let client = UdpClient::new(
        conn.clone(),
        2000,
        audio_config.clone()
    )?;

    let _streams = audio_interface.build_streams(&mode, input_producer, output_consumer)?;

    match mode {
        Mode::Send => {
            let mut client1 = client.clone();
            let send_handle = thread::spawn(move || {
                client1.send_loop(input_consumer);
            });

            send_handle.join().unwrap();
        },
        Mode::Return => {
            let mut client2 = client.clone();
            let recv_handle = thread::spawn(move || {
                client2.recv_loop(output_producer);
            });
        
            recv_handle.join().unwrap();
        }
    }

    Ok(())
}