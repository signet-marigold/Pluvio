use std::time::Duration;
use rodio::Source;

const CHANNELS: u16 = 2;
const SAMPLE_RATE: u32 = 44100;

// Noise generators implementation
pub struct WhiteNoise;

impl WhiteNoise {
  pub fn new() -> Self {
    WhiteNoise
  }
}

impl Iterator for WhiteNoise {
  type Item = f32;

  fn next(&mut self) -> Option<f32> {
    Some(rand::random::<f32>() * 2.0 - 1.0)
  }
}

pub struct BrownNoise {
  last_value: f32,
}

impl BrownNoise {
  pub fn new() -> Self {
    BrownNoise { last_value: 0.0 }
  }
}

impl Iterator for BrownNoise {
  type Item = f32;

  fn next(&mut self) -> Option<f32> {
    let white = rand::random::<f32>() * 2.0 - 1.0;
    self.last_value = (self.last_value + white * 0.02).clamp(-0.5, 0.5);
    Some(self.last_value)
  }
}

pub struct PinkNoise {
  b0: f32,
  b1: f32,
  b2: f32,
  b3: f32,
  b4: f32,
  b5: f32,
  b6: f32,
}

impl PinkNoise {
  pub fn new() -> Self {
    PinkNoise {
      b0: 0.0,
      b1: 0.0,
      b2: 0.0,
      b3: 0.0,
      b4: 0.0,
      b5: 0.0,
      b6: 0.0,
    }
  }
}

impl Iterator for PinkNoise {
  type Item = f32;

  fn next(&mut self) -> Option<f32> {
    let white = rand::random::<f32>() * 2.0 - 1.0;

    self.b0 = 0.99886 * self.b0 + white * 0.0555179;
    self.b1 = 0.99332 * self.b1 + white * 0.0750759;
    self.b2 = 0.96900 * self.b2 + white * 0.1538520;
    self.b3 = 0.86650 * self.b3 + white * 0.3104856;
    self.b4 = 0.55000 * self.b4 + white * 0.5329522;
    self.b5 = -0.7616 * self.b5 - white * 0.0168980;

    let output = self.b0 + self.b1 + self.b2 + self.b3 + self.b4 + self.b5 + self.b6 + white * 0.5362;
    self.b6 = white * 0.115926;

    Some(output * 0.11) // scale to prevent clipping
  }
}

pub struct RainNoise {
  buffer: Vec<f32>,
  position: usize,
}

impl RainNoise {
  pub fn new() -> Self {
    // Create a buffer of rain-like sounds
    let mut buffer = vec![0.0; SAMPLE_RATE as usize]; // 1 second buffer
    for i in 0..SAMPLE_RATE as usize {
      buffer[i] = (rand::random::<f32>() - 0.5) *
                  (1.0 - (i as f32 / SAMPLE_RATE as f32)).powf(2.0);
    }

    RainNoise {
      buffer,
      position: 0,
    }
  }
}

impl Iterator for RainNoise {
  type Item = f32;

  fn next(&mut self) -> Option<f32> {
    let sample = self.buffer[self.position];
    self.position = (self.position + 1) % self.buffer.len();
    Some(sample)
  }
}

// Implement Source trait for WhiteNoise
impl Source for WhiteNoise {
  fn current_frame_len(&self) -> Option<usize> {
    None
  }

  fn channels(&self) -> u16 {
    CHANNELS
  }

  fn sample_rate(&self) -> u32 {
    SAMPLE_RATE
  }

  fn total_duration(&self) -> Option<Duration> {
    None
  }
}

// Implement Source trait for BrownNoise
impl Source for BrownNoise {
  fn current_frame_len(&self) -> Option<usize> {
    None
  }

  fn channels(&self) -> u16 {
    CHANNELS
  }

  fn sample_rate(&self) -> u32 {
    SAMPLE_RATE
  }

  fn total_duration(&self) -> Option<Duration> {
    None
  }
}

// Implement Source trait for PinkNoise
impl Source for PinkNoise {
  fn current_frame_len(&self) -> Option<usize> {
    None
  }

  fn channels(&self) -> u16 {
    CHANNELS
  }

  fn sample_rate(&self) -> u32 {
    SAMPLE_RATE
  }

  fn total_duration(&self) -> Option<Duration> {
    None
  }
}

// Implement Source trait for RainNoise
impl Source for RainNoise {
  fn current_frame_len(&self) -> Option<usize> {
    None
  }

  fn channels(&self) -> u16 {
    CHANNELS
  }

  fn sample_rate(&self) -> u32 {
    SAMPLE_RATE
  }

  fn total_duration(&self) -> Option<Duration> {
    None
  }
}
