
use std::thread;
use std::net::UdpSocket;
use std::sync::Arc;

use anyhow::Result;
use clap::{Arg, App};

use p2p_audio::udp::client::{UdpClient, UdpClientConfig};
use p2p_audio::audio::{AudioInterface, AudioConfig};
use p2p_audio::util::Mode;
use p2p_audio::ringbuffer;

fn main() {
    let matches = App::new("")
        .arg(Arg::with_name("mode")
            .value_name("MODE")
            .index(1)
            .required(true)
            .possible_values(&["send", "return"]))
        .arg(Arg::with_name("channel")
            .value_name("CHANNEL")
            .short("c")
            .long("channel")
            .takes_value(true)
            .default_value("1"))
        .arg(Arg::with_name("stereo")
            .value_name("STEREO")
            .long("stereo")
            .takes_value(false))
        .arg(Arg::with_name("sample_rate")
            .value_name("SAMPLE_RATE")
            .short("s")
            .long("sample-rate")
            .takes_value(true)
            .default_value("44100"))
        .arg(Arg::with_name("buffer_size")
            .value_name("BUFFER_SIZE")
            .short("b")
            .long("buffer-size")
            .takes_value(true)
            .default_value("128"))
        .arg(Arg::with_name("input")
            .value_name("INPUT_DEVICE")
            .short("i")
            .long("input")
            .takes_value(true)
            .default_value("default"))
        .arg(Arg::with_name("output")
            .value_name("OUTPUT_DEVICE")
            .short("o")
            .long("output")
            .takes_value(true)
            .default_value("default"))
        .arg(Arg::with_name("port")
            .value_name("PORT")
            .short("p")
            .long("port")
            .takes_value(true)
            .default_value("0"))
        .arg(Arg::with_name("remote")
            .value_name("REMOTE")
            .short("r")
            .long("remote")
            .takes_value(true)
            .required(true))
        .get_matches();
    
    let mode = match matches.value_of("mode").unwrap() {
        "send" => Mode::Send,
        "return" => Mode::Return,
        _ => {
            return;
        }
    };

    let channel = matches.value_of("channel").unwrap();
    let stereo = matches.is_present("stereo");
    let sample_rate = matches.value_of("sample_rate").unwrap();
    let buffer_size = matches.value_of("buffer_size").unwrap();
    let resolution = 16;
    let input_device = matches.value_of("input").unwrap();
    let output_device = matches.value_of("output").unwrap();

    let audio_config = AudioConfig::new(
        stereo,
        channel,
        sample_rate,
        buffer_size,
        resolution,
        input_device,
        output_device,
    );

    let port = matches.value_of("port").unwrap();
    let remote = matches.value_of("remote").unwrap();

    let client_config = UdpClientConfig::new(port, remote);

    match run(mode, audio_config, client_config) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err)
    }
}

fn run(mode: Mode, audio_config: AudioConfig, client_config: UdpClientConfig) -> Result<()> {
    let local_addr = format!("127.0.0.1:{}", client_config.port);
    let conn = UdpSocket::bind(&local_addr)?;

    let conn = Arc::new(conn);

    conn.connect(client_config.remote)?;

    run_stream(mode, conn, audio_config)
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