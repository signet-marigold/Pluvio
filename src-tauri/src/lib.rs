use std::thread;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{State, AppHandle, Manager, Emitter};
use rodio::{Decoder, Sink, Source, OutputStream, OutputStreamHandle};
use tokio::time::Duration;
use tokio::sync::mpsc;
use tokio::runtime::Runtime;
use ringbuf::{HeapRb, Consumer, Producer};

mod intercept;
mod noisegen;
mod capture;


fn start_audio_thread(producer: Producer<f32>) -> cpal::Stream {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data {
                producer.push(*sample).ok();
            }
        },
        |err| eprintln!("Audio stream error: {}", err),
        None
    ).unwrap();

    stream.play().unwrap();
    stream
}


fn fade_volume(sink: Arc<Sink>, target_volume: f32, duration: Duration, cancel_flag: Arc<AtomicBool>) {
  let initial_volume = sink.volume();
  let steps = 20; // Number of steps for the fade
  let step_duration = duration / steps;
  let volume_step = (target_volume - initial_volume) / steps as f32;

  // Reset flag before the loop
  // This is asynchronous so we are looking for changes
  // to the flag after fade volume has been called
  cancel_flag.store(false, Ordering::Relaxed);

  for i in 0..steps {
    if cancel_flag.load(Ordering::Relaxed) {
      println!("Fade canceled");
      cancel_flag.store(false, Ordering::Relaxed); // Reset the flag
      break;
    }

    let new_volume = initial_volume + volume_step * (i + 1) as f32;
    sink.set_volume(new_volume);
    thread::sleep(step_duration);
  }

  // Ensure the final volume is exactly the target volume
  sink.set_volume(target_volume);
  cancel_flag.store(false, Ordering::Relaxed); // Reset the flag
}


// Make sure the ui lines up with these init values
// Value between 0.0(min) - 1.0(max)
const INIT_MASTER_VOLUME: f32 = 0.85;
const INIT_TRACK_VOLUME: f32 = 0.3;
const VOLUME_CHANGE_DURATION: u64 = 100;

struct AudioTrack {
  sink: Arc<Sink>,
  base_volume: f32,
}

struct AudioState {
  producer: Producer<f32>,
}



struct AppState {
  tracks: Mutex<HashMap<String, AudioTrack>>,
  master_volume: Arc<Mutex<f32>>,
  stream_handle: Arc<OutputStreamHandle>,
  cancel_flag: Arc<AtomicBool>,
  runtime: Runtime, // Global Tokio runtime
  audio_state: Mutex<AudioState>,
}

// Implement thread safety
unsafe impl Send for AudioState {}
unsafe impl Sync for AudioState {}


#[tauri::command]
async fn add_file_track(
  id: String,
  file_path: String,
  state: State<'_, AppState>,
  app_handle: AppHandle,
) -> Result<(), String> {
  let stream_handle = &state.stream_handle;
  let sink = Sink::try_new(stream_handle).map_err(|e| e.to_string())?;

  // Open the audio file
  let file = File::open(&file_path).map_err(|e| e.to_string())?;
  let source = Decoder::new(file).map_err(|e| e.to_string())?;

  // Create a channel to send samples to the frontend
  let (sample_tx, mut sample_rx) = mpsc::channel(100);

  // Wrap the source in Buffered and create the intercepting source
  let intercepting_source = intercept::InterceptingSource::new(source.convert_samples().buffered(), sample_tx);

  // Use the global runtime to spawn the task
  state.runtime.spawn(async move {
    let mut sample_rx = sample_rx;
    while let Some(sample) = sample_rx.recv().await {
      let _ = app_handle.emit("audio_sample", sample); // Emit the sample to the frontend
    }
  });

  // Append the source to the sink and loop it
  sink.append(intercepting_source.buffered().repeat_infinite());
  sink.pause();

  // Store the track
  let mut tracks = state.tracks.lock().unwrap();
  tracks.insert(id, AudioTrack {
    sink: Arc::new(sink),
    base_volume: 1.0,
  });

  Ok(())
}


#[tauri::command]
async fn add_noise_track(
  id: String,
  noise_type: String,
  state: State<'_, AppState>,
  app_handle: AppHandle,
) -> Result<(), String> {
  println!("Adding track: {}", id);

  let sink = Sink::try_new(&state.stream_handle).map_err(|e| e.to_string())?;

  let source: Box<dyn Source<Item = f32> + Send + Sync> = match noise_type.as_str() {
    "white" => Box::new(noisegen::WhiteNoise::new().repeat_infinite()),
    "brown" => Box::new(noisegen::BrownNoise::new().repeat_infinite()),
    "pink" => Box::new(noisegen::PinkNoise::new().repeat_infinite()),
    "rain" => Box::new(noisegen::RainNoise::new().repeat_infinite()),
    _ => return Err("Invalid noise type".into()),
  };

  // Create a channel to send samples to the frontend
  let (sample_tx, mut sample_rx) = mpsc::channel(100);

  // Wrap the source in Buffered and create the intercepting source
  let intercepting_source = intercept::InterceptingSource::new(source.convert_samples().buffered(), sample_tx);

  // Use the global runtime to spawn the task
  state.runtime.spawn(async move {
    let mut sample_rx = sample_rx;
    while let Some(sample) = sample_rx.recv().await {
      let _ = app_handle.emit("audio_sample", sample); // Emit the sample to the frontend
    }
  });

  sink.append(intercepting_source);
  sink.pause();
  println!("Sink is playing for track: {}", id); // Log sink playback

  // Set init volume
  sink.set_volume(INIT_TRACK_VOLUME * INIT_MASTER_VOLUME);

  let mut tracks = state.tracks.lock().unwrap();
  tracks.insert(id, AudioTrack {
    sink: Arc::new(sink),
    base_volume: INIT_TRACK_VOLUME,
  });

  Ok(())
}

#[tauri::command]
async fn set_volume(
  id: String,
  volume: f32,
  state: State<'_, AppState>,
) -> Result<(), String> {
  let master_volume = *state.master_volume.lock().unwrap();
  let mut tracks = state.tracks.lock().unwrap();

  if let Some(track) = tracks.get_mut(&id) {
    // Cancel any ongoing fade
    state.cancel_flag.store(true, Ordering::Relaxed);

    let target_volume = volume * master_volume;
    let sink = Arc::clone(&track.sink); // Use a shared reference to the Sink
    let cancel_flag = Arc::clone(&state.cancel_flag); // Clone the atomic flag

    // Spawn a thread to perform the fade
    thread::spawn(move || {
      fade_volume(sink, target_volume, Duration::from_millis(VOLUME_CHANGE_DURATION), cancel_flag);
    });

    track.base_volume = volume;
    println!("Volume for track {} set to: {}", id, volume);
  } else {
    return Err(format!("Track {} not found", id));
  }

  Ok(())
}

#[tauri::command]
async fn set_master_volume(
  volume: f32,
  state: State<'_, AppState>,
) -> Result<(), String> {
  // Update the master volume
  let mut master_volume = state.master_volume.lock().unwrap();
  *master_volume = volume;

  // Update all track volumes with a smooth transition
  let mut tracks = state.tracks.lock().unwrap();
  for (_, track) in tracks.iter_mut() {
    // Cancel any ongoing fade
    state.cancel_flag.store(true, Ordering::Relaxed);

    let target_volume = track.base_volume * volume;
    let sink = Arc::clone(&track.sink); // Use a shared reference to the Sink
    let cancel_flag = Arc::clone(&state.cancel_flag); // Clone the atomic flag

    // Spawn a thread to perform the fade
    thread::spawn(move || {
      fade_volume(sink, target_volume, Duration::from_millis(VOLUME_CHANGE_DURATION), cancel_flag);
    });
  }

  println!("Master volume set to: {}", volume);
  Ok(())
}

#[tauri::command]
async fn pause_track(
  id: String,
  state: State<'_, AppState>,
) -> Result<(), String> {
  let mut tracks = state.tracks.lock().unwrap();

  if let Some(track) = tracks.get_mut(&id) {
    track.sink.pause();
    println!("Paused track: {}", id);
  } else {
    return Err(format!("Track {} not found", id));
  }

  Ok(())
}

#[tauri::command]
async fn resume_track(
  id: String,
  state: State<'_, AppState>,
) -> Result<(), String> {
  let mut tracks = state.tracks.lock().unwrap();

  if let Some(track) = tracks.get_mut(&id) {
    track.sink.play();
    println!("Resumed track: {}", id);
  } else {
    return Err(format!("Track {} not found", id));
  }

  Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create audio stream");
  let runtime = Runtime::new().expect("Failed to create Tokio runtime"); // Create the runtime;

  let rb = HeapRb::<f32>::new(4096);
  let (producer, consumer) = rb.split();

  // Start audio thread
  let _stream = start_audio_thread(producer);

  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      let handle = app.handle();

            // Spawn waveform processing task
            tokio::spawn(async move {
                let mut consumer = consumer;
                let mut buffer = Vec::with_capacity(512);

                loop {
                    while let Some(sample) = consumer.pop() {
                        buffer.push(sample);
                        if buffer.len() >= 512 {
                            handle.emit("waveform", &buffer).unwrap();
                            buffer.clear();
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            });

      Ok(())
    })
    .manage(AppState {
      tracks: Mutex::new(HashMap::new()),
      master_volume: Arc::new(Mutex::new(INIT_MASTER_VOLUME)),
      stream_handle: Arc::new(stream_handle),
      cancel_flag: Arc::new(AtomicBool::new(false)),
      runtime,
      audio_state: Mutex::new(AudioState {
        producer: consumer.into_producer()
      })
    })
    .invoke_handler(tauri::generate_handler![
      add_noise_track,
      add_file_track,
      set_volume,
      set_master_volume,
      pause_track,
      resume_track,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
