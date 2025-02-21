use rodio::source::Buffered;
use rodio::Source;
use tokio::sync::mpsc::Sender;

pub struct InterceptingSource<S>
where
  S: Source<Item = f32> + Clone, // Ensure S implements Source<Item = f32>
{
  inner: Buffered<S>, // Use Buffered to make the source cloneable
  sample_rate: u32,
  sender: Sender<f32>,
}

impl<S> InterceptingSource<S>
where
  S: Source<Item = f32> + Clone, // Ensure S implements Source<Item = f32>
{
  pub fn new(inner: S, sender: Sender<f32>) -> Self {
    let sample_rate = inner.sample_rate();
    println!("Creating InterceptingSource with sample rate: {}", sample_rate); // Log creation
    InterceptingSource {
      inner: inner.buffered(), // Wrap the source in Buffered
      sample_rate,
      sender,
    }
  }
}

impl<S> Iterator for InterceptingSource<S>
where
  S: Source<Item = f32> + Clone,
{
  type Item = f32;

  fn next(&mut self) -> Option<Self::Item> {
    println!("InterceptingSource::next called"); // Log iterator call
    if let Some(sample) = self.inner.next() {
      println!("Intercepted sample: {}", sample); // Log intercepted sample
      // Send the sample to the frontend
      let _ = self.sender.try_send(sample); // Use try_send to avoid blocking
      Some(sample)
    } else {
      None
    }
  }
}

impl<S> Source for InterceptingSource<S>
where
  S: Source<Item = f32> + Clone,
{
  fn current_frame_len(&self) -> Option<usize> {
    self.inner.current_frame_len()
  }

  fn channels(&self) -> u16 {
    self.inner.channels()
  }

  fn sample_rate(&self) -> u32 {
    self.inner.sample_rate()
  }

  fn total_duration(&self) -> Option<std::time::Duration> {
    self.inner.total_duration()
  }
}
