use ringbuf::{RingBuffer, Consumer, Producer};

pub fn create(packet_buffer_size: usize)
-> (Producer<f32>, Consumer<f32>, Producer<f32>, Consumer<f32>)
{
    let ringbuffer_size = packet_buffer_size * 4;
    let input_buffer = RingBuffer::new(ringbuffer_size);
    let output_buffer = RingBuffer::new(ringbuffer_size);

    let (mut input_producer, input_consumer) = input_buffer.split();
    let (mut output_producer, output_consumer) = output_buffer.split();

    for _ in 0..ringbuffer_size {
        input_producer.push(0.0).unwrap();
    }

    for _ in 0..ringbuffer_size {
        output_producer.push(0.0).unwrap();
    }

    (input_producer, input_consumer, output_producer, output_consumer)
}