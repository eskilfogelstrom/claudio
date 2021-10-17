
use std::net::UdpSocket;
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use ringbuf::{Producer, Consumer};

use crate::udp::packet::{Packet, MessageType};
use crate::audio::AudioConfig;

pub struct UdpClientConfig {
    pub remote: String,
    pub port: String
}

impl UdpClientConfig {
    pub fn new(port: &str, remote: &str) -> Self {
        Self {
            port: port.to_string(),
            remote: remote.to_string()
        }
    } 
}

#[derive(Clone)]
pub struct UdpClient {
    conn: Arc<UdpSocket>,
    send_sequence_number: u16,
    recv_sequence_number: Option<u16>,
    redundancy: u16,
    send_packet_queue: VecDeque<Packet>,
    audio_config: AudioConfig
}

impl UdpClient {
    pub fn new(
        conn: Arc<UdpSocket>,
        sequence_number: u16,
        audio_config: AudioConfig,
    ) -> Result<Self> {
        let redundancy = 3u16;
        let send_packet_queue = VecDeque::with_capacity(redundancy as usize);
        let send_sequence_number = sequence_number;
        let recv_sequence_number = Option::None;

        let client = Self {
            conn,
            send_sequence_number,
            recv_sequence_number,
            redundancy,
            audio_config,
            send_packet_queue
        };

        Ok(client)
    }

    fn get_packet_size(&self) -> usize {
        Packet::get_header_size() + (self.audio_config.get_frame_size() * 4)
    }

    fn get_redundant_packet_size(&self) -> usize {
        self.get_packet_size() * self.redundancy as usize
    }

    pub fn send(&mut self, samples: &[f32]) -> Result<()> {
        self.send_sequence_number += 1;

        let packet = Packet::new(
            MessageType::Audio,
            self.send_sequence_number,
            0,
            3,
            self.audio_config.sample_rate,
            self.audio_config.get_channel_count(),
            self.audio_config.buffer_size,
            samples.to_vec(),
        );

        if self.send_packet_queue.len() == 3 {
            self.send_packet_queue.pop_back();
        }
        self.send_packet_queue.push_front(packet);

        let mut buffer = Vec::with_capacity(self.get_redundant_packet_size());

        for packet in &self.send_packet_queue {
            buffer.append(&mut packet.to_buffer())
        }

        self.conn.send(&buffer)?;

        Ok(())
    }
    
    pub fn recv(&mut self) -> VecDeque<Packet> {
        let mut buffer = vec![0u8; self.get_redundant_packet_size()];

        match self.conn.recv(&mut buffer) {
            Err(e) => eprintln!("Error receiving packet: {}", e),
            _ => ()
        };

        if buffer.len() < self.get_packet_size() {
            return VecDeque::<Packet>::new();
        }

        let first_packet_buffer = &buffer[0..self.get_packet_size()];
        let first_packet = Packet::from_buffer(first_packet_buffer);


        let new_sequence_number = first_packet.sequence_number;
        let mut current_sequence_number = new_sequence_number;

        let mut packets = VecDeque::with_capacity(self.redundancy as usize);
        packets.push_front(first_packet);

        for i in 1..(self.redundancy - 1) {
            let prev_sequence_number = match self.recv_sequence_number {
                Some(s) => s,
                None => current_sequence_number - 1
            };

            if prev_sequence_number + 1 == current_sequence_number {
                break;
            }
            
            let packet_buffer = &buffer[self.get_packet_size() * i as usize..self.get_packet_size() * (i + 1) as usize];
            
            let packet = Packet::from_buffer(packet_buffer);
            current_sequence_number -= 1;
            packets.push_front(packet);
        }

        self.recv_sequence_number = Option::Some(new_sequence_number);

        packets
    }

    pub fn send_loop(&mut self, mut input_consumer: Consumer<f32>) {
        println!("Sending...");
        loop {
            if input_consumer.len() < self.audio_config.get_frame_size() {
                continue;
            }

            let mut buffer = vec![0f32; self.audio_config.get_frame_size()];
            input_consumer.pop_slice(&mut buffer);
            
            match self.send(&buffer) {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err)
            };
        }
    }

    pub fn recv_loop(&mut self, mut output_producer: Producer<f32>) {
        println!("Receiving...");
        loop {
            let packets = self.recv();
            let mut samples = Vec::with_capacity(self.audio_config.get_frame_size() * packets.len());

            for packet in packets {
                samples.extend_from_slice(&packet.audio_samples);
            }

            output_producer.push_slice(&samples);
        }
    }
}