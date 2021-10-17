extern crate cpal;

use std::collections::HashMap;

use cpal::{
    StreamConfig,
    SupportedBufferSize,
    BufferSize,
    Device,
    SampleRate,
    SupportedStreamConfigRange,
    Stream
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{Producer, Consumer};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

use crate::util::Mode;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioConfig {
    host: String,
    pub input_device: String,
    pub output_device: String,
    pub sample_rate: u32,
    pub buffer_size: u32,
    stereo: bool,
    pub input_channel: u32,
    pub output_channel: u32,
}

impl AudioConfig {
    pub fn new(
        host: String,
        input_device: String,
        output_device: String,
        sample_rate: u32,
        buffer_size: u32,
        stereo: bool,
        input_channel: u32,
        output_channel: u32,
    ) -> Self {

        let config = Self {
            host,
            stereo,
            input_channel,
            output_channel,
            sample_rate,
            buffer_size,
            input_device,
            output_device
        };

        config
    }

    pub fn get_channel_count(&self) -> u32 {
        match self.stereo {
            true => 2,
            false => 1
        }
    }

    pub fn get_frame_size(&self) -> usize {
        (self.buffer_size * self.get_channel_count()) as usize
    }

}

impl Default for AudioConfig {
    fn default() -> Self {
        let host = cpal::default_host();
        let input_device = host.default_input_device().unwrap();
        let output_device = host.default_output_device().unwrap();

        AudioConfig::new(
            host.id().name().to_string(),
            input_device.name().unwrap(),
            output_device.name().unwrap(),
            44100,
            128,
            false,
            0,
            0,
        )
    }
}

#[derive(Serialize, Debug)]
pub struct Buffer {
    min: u32,
    max: u32,
}

#[derive(Serialize, Debug)]
pub struct SupportedConfig {
    sample_rates: Vec<u32>,
    buffer_size: Buffer,
    channels: u16
}

pub struct AudioInterface {
    audio_config: AudioConfig,
    input_device: Device,
    output_device: Device,
    input_config: StreamConfig,
    output_config: StreamConfig,
}

impl AudioInterface {
    pub fn validate_config(config: &AudioConfig) -> Result<()> {
        let host = cpal::default_host();

        let mut devices = host.devices()?;

        let input_device = devices.find(|device| device.name().unwrap() == config.input_device).ok_or(anyhow!("Could not find input device"))?;

        let output_device = devices.find(|device| device.name().unwrap() == config.output_device).ok_or(anyhow!("Could not find output device"))?;
        
        let supported_input_configs = input_device.supported_input_configs()?;
        let _input_config = AudioInterface::get_config(supported_input_configs, config.sample_rate, config.buffer_size, config.input_channel + config.get_channel_count())?;
        
        let supported_output_configs = output_device.supported_output_configs()?;
        let _output_config = AudioInterface::get_config(supported_output_configs, config.sample_rate, config.buffer_size, config.output_channel + config.get_channel_count())?;

        Ok(())
    }

    pub fn get_supported_configs() -> Result<HashMap<String, HashMap<String, HashMap<String, SupportedConfig>>>> {
        
        let available_hosts = cpal::available_hosts();

        let mut val = HashMap::new();

        for host_id in available_hosts {
            let host = cpal::host_from_id(host_id)?;
            let devices = host.devices()?;
            let mut host_map = HashMap::new();

            for device in devices {
                let mut device_map = HashMap::new();

                // let mut input_configs = Vec::new();
                let supported_input_configs: Vec<SupportedStreamConfigRange> = device.supported_input_configs()?.collect();

                // let mut output_configs = Vec::new();
                let supported_output_configs: Vec<SupportedStreamConfigRange> = device.supported_output_configs()?.collect();

              
                let mut sample_rates = Vec::new();
                let mut channels = 0;
                let mut buffer_size = Buffer { min: 0, max: 0};

                for config in supported_input_configs {
                    let buffer_size1 = match config.buffer_size() {
                        SupportedBufferSize::Range { min, max} => Buffer { min: *min, max: *max },
                        SupportedBufferSize::Unknown => return Err(anyhow!("Could not get buffer size")),
                    };
                    sample_rates.push(config.max_sample_rate().0);
                    channels = config.channels();
                    buffer_size = buffer_size1;
                }

                let input_config = SupportedConfig {
                    sample_rates: sample_rates,
                    channels,
                    buffer_size
                };

                let mut sample_rates = Vec::new();
                let mut channels = 0;
                let mut buffer_size = Buffer { min: 0, max: 0};

                for config in supported_output_configs {
                    let buffer_size1 = match config.buffer_size() {
                        SupportedBufferSize::Range { min, max} => Buffer { min: *min, max: *max },
                        SupportedBufferSize::Unknown => return Err(anyhow!("Could not get buffer size")),
                    };
                    sample_rates.push(config.max_sample_rate().0);
                    channels = config.channels();
                    buffer_size = buffer_size1;
                }

                let output_config = SupportedConfig {
                    sample_rates: sample_rates,
                    channels,
                    buffer_size
                };

                device_map.insert("input".to_string(), input_config);
                device_map.insert("output".to_string(), output_config);

                host_map.insert(device.name().unwrap(), device_map);
            }

            val.insert(host_id.name().to_string(), host_map);
        }

        Ok(val)
    }

    fn get_config<T>(mut supported_configs: T, sample_rate: u32, buffer_size: u32, channel: u32) -> Result<StreamConfig>
    where T: Iterator<Item=SupportedStreamConfigRange> {
        let supported_config = supported_configs.find(|config_range| {
            let (min_buffer_size, max_buffer_size) = match config_range.buffer_size() {
                SupportedBufferSize::Range { min, max } => (*min, *max),
                SupportedBufferSize::Unknown => (0, 0),
            };
                sample_rate >= config_range.min_sample_rate().0 &&
                sample_rate <= config_range.max_sample_rate().0 &&
                channel <= config_range.channels() as u32 &&
                buffer_size >= min_buffer_size &&
                buffer_size <= max_buffer_size
        }).ok_or(anyhow!("Could not find supported config"))?;

        let mut stream_config: StreamConfig = supported_config.with_sample_rate(SampleRate(sample_rate)).into();
        stream_config.buffer_size = BufferSize::Fixed(buffer_size);

        Ok(stream_config)
    }

    pub fn new(config: AudioConfig) -> Result<AudioInterface> {
        let host = cpal::default_host();

        let mut devices = host.devices()?.into_iter();
        
        let input_device = devices.find(|device| device.name().unwrap() == config.input_device).ok_or(anyhow!("Could not find input device"))?;

        let output_device = devices.find(|device| device.name().unwrap() == config.output_device).ok_or(anyhow!("Could not find output device"))?;
        
        let supported_input_configs = input_device.supported_input_configs()?;
        let input_config = AudioInterface::get_config(supported_input_configs, config.sample_rate, config.buffer_size, config.input_channel + config.get_channel_count())?;
        
        let supported_output_configs = output_device.supported_output_configs()?;
        let output_config = AudioInterface::get_config(supported_output_configs, config.sample_rate, config.buffer_size, config.output_channel + config.get_channel_count())?;

        let audio_interface = AudioInterface {
            audio_config: config,
            input_device,
            output_device,
            input_config,
            output_config,
        };

        Ok(audio_interface)
    }

    pub fn build_streams(
        &self,
        mode: &Mode,
        mut input_producer: Producer<f32>,
        mut output_consumer: Consumer<f32>
    ) -> Result<(Stream)> {
        
        match mode {
            Mode::Send => {
                let start = self.audio_config.input_channel;
                let end = self.audio_config.input_channel + self.audio_config.get_channel_count();
                let channels = self.input_config.channels as u32;

                let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut output_fell_behind = false;
                    let mut count = 0; 
                    
                    for &sample in data {
                        if count >= start && count < end {
                            if input_producer.push(sample).is_err() {
                                output_fell_behind = true;
                            }
                        }
                        count += 1;

                        if count >= channels {
                            count = 0;
                        }
                    }
                    if output_fell_behind {
                        // eprintln!("output stream fell behind: try increasing latency");
                    }
                };

                let input_stream = self.input_device.build_input_stream(&self.input_config, input_data_fn, err_fn)?;

                input_stream.play()?;

                return Ok(input_stream);
            },
            Mode::Return => {
                let start = self.audio_config.output_channel;
                let end = self.audio_config.output_channel + self.audio_config.get_channel_count();
                let channels = self.output_config.channels as u32;

                let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut input_fell_behind = false;
                    let mut count = 0;

                    for sample in data {
                        if count >= start && count < end {
                            *sample = match output_consumer.pop() {
                                Some(s) => s,
                                None => {
                                    input_fell_behind = true;
                                    0.0
                                }
                            };
                        } else {
                            *sample = 0.0;
                        }
                        count += 1;

                        if count >= channels {
                            count = 0;
                        }
                    }
                    if input_fell_behind {
                        // eprintln!("input stream fell behind: try increasing latency");
                    }
                };

                let output_stream = self.output_device.build_output_stream(&self.output_config, output_data_fn, err_fn)?;

                output_stream.play()?;

                return Ok(output_stream);
            }
        };
        

        // Ok((input_stream, output_stream))
    }
}


fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}