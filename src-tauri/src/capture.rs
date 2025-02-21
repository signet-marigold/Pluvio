use cpal::{
  traits::{DeviceTrait, HostTrait, StreamTrait},
  Stream,
};
use ringbuf::HeapRb;
use std::sync::Arc;

pub struct AudioCapture {
  stream: Stream,
  rb: Arc<HeapRb<f32>>,
}

impl AudioCapture {
  pub fn new() -> Result<Self, anyhow::Error> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config()?;

    let rb = Arc::new(HeapRb::<f32>::new(1024 * 16)); // 16k buffer
    let (prod, cons) = rb.split();

    let stream = device.build_output_stream(
      &config.into(),
      move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        // Audio callback - runs in real-time thread
        for sample in data {
          prod.push(*sample).ok();
        }
      },
      |err| eprintln!("Audio stream error: {}", err),
      None
    )?;

    Ok(Self { stream, rb })
  }
}
