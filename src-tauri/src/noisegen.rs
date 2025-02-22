// Generate audio noise on the fly
//
// Currently locked to 2 channels but could be expanded to more.
// Noise volume is scaled so that the apparent 'loudness' if each is close to the same.
// Brown and Pink noise generates two streams for stereo mode.
// White noise is automatically stereo by the nature of it.
// Not using previous data to generate, filling the channels as new data is
// generated is just as good as having two separate generators.

const SAMPLE_RATE: u32 = 44100;

// Scale noise for the same aparent volume
const SCALE_WHITE_NOISE: f32 = 0.2;
const SCALE_BROWN_NOISE: f32 = 1.0;
const SCALE_PINK_NOISE: f32 = 0.55;

// White Noise Generator
// No need for multiple generators for stereo
pub struct WhiteNoise;

impl WhiteNoise {
  pub fn new() -> Self {
    WhiteNoise
  }
}

impl Iterator for WhiteNoise {
  type Item = f32;

  fn next(&mut self) -> Option<f32> {
    Some((rand::random::<f32>() * 2.0 - 1.0) * SCALE_WHITE_NOISE)
  }
}

impl rodio::Source for WhiteNoise {
  fn current_frame_len(&self) -> Option<usize> { None }
  fn channels(&self) -> u16 { 2 }
  fn sample_rate(&self) -> u32 { SAMPLE_RATE }
  fn total_duration(&self) -> Option<std::time::Duration> { None }
}

// Brownian Noise Generator
pub struct BrownNoise {
  buffer: [f32; 2],
  channel_flag: bool,
}

impl BrownNoise {
  pub fn new() -> Self {
    Self {
      buffer: [0.0; 2],
      channel_flag: false,
    }
  }

  fn generate_sample(&mut self, channel_id: usize) -> f32 {
    let white = rand::random::<f32>() * 2.0 - 1.0;

    self.buffer[channel_id] = (self.buffer[channel_id] + white * 0.02).clamp(-0.5, 0.5);

    self.buffer[channel_id]
  }
}

impl Iterator for BrownNoise {
  type Item = f32;

  fn next(&mut self) -> Option<f32> {
    let sample = if self.channel_flag {
      self.generate_sample(1)
    } else {
      self.generate_sample(0)
    };

    self.channel_flag = !self.channel_flag;

    Some(sample * SCALE_BROWN_NOISE)
  }
}

impl rodio::Source for BrownNoise {
  fn current_frame_len(&self) -> Option<usize> { None }
  fn channels(&self) -> u16 { 2 }
  fn sample_rate(&self) -> u32 { SAMPLE_RATE }
  fn total_duration(&self) -> Option<std::time::Duration> { None }
}

// Pink Noise Generator
pub struct PinkNoise {
  buffer: [[f32; 7]; 2],
  channel_flag: bool,
}

impl PinkNoise {
  pub fn new() -> Self {
    PinkNoise {
      buffer: [[0.0; 7]; 2],
      channel_flag: false,
    }
  }

  fn generate_sample(&mut self, channel_id: usize) -> f32 {
    let white = rand::random::<f32>() * 2.0 - 1.0;
    // Magic vodo
    self.buffer[channel_id][0] = 0.99886 * self.buffer[channel_id][0] + white * 0.0555179;
    self.buffer[channel_id][1] = 0.99332 * self.buffer[channel_id][1] + white * 0.0750759;
    self.buffer[channel_id][2] = 0.96900 * self.buffer[channel_id][2] + white * 0.1538520;
    self.buffer[channel_id][3] = 0.86650 * self.buffer[channel_id][3] + white * 0.3104856;
    self.buffer[channel_id][4] = 0.55000 * self.buffer[channel_id][4] + white * 0.5329522;
    self.buffer[channel_id][5] = -0.7616 * self.buffer[channel_id][5] - white * 0.0168980;

    let output = self.buffer[channel_id].iter().sum::<f32>() + white * 0.5362;
    self.buffer[channel_id][6] = white * 0.115926;

    output * 0.11
  }
}

impl Iterator for PinkNoise {
  type Item = f32;

  fn next(&mut self) -> Option<f32> {
    let sample = if self.channel_flag {
      self.generate_sample(1)
    } else {
      self.generate_sample(0)
    };

    self.channel_flag = !self.channel_flag;

    Some(sample * SCALE_PINK_NOISE)
  }
}

impl rodio::Source for PinkNoise {
  fn current_frame_len(&self) -> Option<usize> { None }
  fn channels(&self) -> u16 { 2 }
  fn sample_rate(&self) -> u32 { SAMPLE_RATE }
  fn total_duration(&self) -> Option<std::time::Duration> { None }
}
