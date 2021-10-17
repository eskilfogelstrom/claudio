#[derive(Copy, Clone, Debug)]
pub enum MessageType {
    Audio
}

impl MessageType {
    fn from_u8(v: u8) -> Option<MessageType> {
        match v {
            0 => Some(MessageType::Audio),
            _ => None
        }
    }
}

#[derive(Clone)]
pub struct Packet {
    pub message_type: MessageType,
    pub sequence_number: u16,
    pub timestamp: u32,
    pub redundancy: u8,
    pub sample_rate: u32,
    pub channel_count: u32,
    pub buffer_size: u32,
    pub audio_samples: Vec<f32>,
}

impl Packet {
    pub fn new(
        message_type: MessageType,
        sequence_number: u16,
        timestamp: u32,
        redundancy: u8,
        sample_rate: u32,
        channel_count: u32,
        buffer_size: u32,
        audio_samples: Vec<f32>
    ) -> Packet {
        Packet {
            message_type,
            sequence_number,
            timestamp,
            redundancy,
            sample_rate,
            channel_count,
            buffer_size,
            audio_samples
        }
    }

    pub fn get_header_size() -> usize {
        17
    }

    fn get_header_buffer(&self) -> Vec<u8> {
        let mut buf_header = Vec::with_capacity(19);
        buf_header.push(self.message_type as u8);
        
        let sequence_number = self.sequence_number.to_be_bytes();
        buf_header.push(sequence_number[0]);
        buf_header.push(sequence_number[1]);

        let timestamp = self.timestamp.to_be_bytes();
        buf_header.push(timestamp[0]);
        buf_header.push(timestamp[1]);
        buf_header.push(timestamp[2]);
        buf_header.push(timestamp[3]);

        buf_header.push(self.redundancy);

        let sample_rate = self.sample_rate.to_be_bytes();
        buf_header.push(sample_rate[0]);
        buf_header.push(sample_rate[1]);
        buf_header.push(sample_rate[2]);
        buf_header.push(sample_rate[3]);

        buf_header.push(self.channel_count as u8);

        let buffer_size = self.buffer_size.to_be_bytes();
        buf_header.push(buffer_size[0]);
        buf_header.push(buffer_size[1]);
        buf_header.push(buffer_size[2]);
        buf_header.push(buffer_size[3]);

        buf_header
    }

    pub fn get_buffer_size(&self) -> usize {
        Packet::get_header_size() + (self.buffer_size * self.channel_count as u32 * 4) as usize
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.get_buffer_size());

        let buf_header = self.get_header_buffer();

        for b in buf_header {
            buffer.push(b);
        }

        for sample in &self.audio_samples {
            let bytes = sample.to_be_bytes();
            buffer.push(bytes[0]);
            buffer.push(bytes[1]);
            buffer.push(bytes[2]);
            buffer.push(bytes[3]);
        }

        buffer
    }

    pub fn from_buffer(buffer: &[u8]) -> Packet {
        let message_type = MessageType::from_u8(buffer[0]).expect("Invalid message type");
        
        let sequence_number = u16::from_be_bytes([buffer[1], buffer[2]]);
        let timestamp = u32::from_be_bytes([
            buffer[3],
            buffer[4],
            buffer[5],
            buffer[6],
        ]);
        let redundancy = buffer[7];
        let sample_rate = u32::from_be_bytes([
            buffer[8],
            buffer[9],
            buffer[10],
            buffer[11],
        ]);
        let channel_count = buffer[12] as u32;
        let buffer_size = u32::from_be_bytes([
            buffer[13],
            buffer[14],
            buffer[15],
            buffer[16],
        ]);

        let mut audio_samples = Vec::with_capacity((buffer_size * channel_count as u32) as usize);
        
        let mut audio_buffer = [0u8; 4];
        let mut count = 0;
        
        for b in &buffer[Packet::get_header_size()..] {
            audio_buffer[count] = *b;
            count += 1;
            if count == 4 {
                audio_samples.push(f32::from_be_bytes(audio_buffer));
                count = 0;
            }
        }

        Packet {
            message_type,
            sequence_number,
            timestamp,
            redundancy,
            sample_rate,
            channel_count,
            buffer_size,
            audio_samples
        }
    }
}