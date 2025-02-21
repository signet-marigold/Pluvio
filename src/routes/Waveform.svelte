<script lang="ts">
  import { onMount } from 'svelte';
  import WaveSurfer from 'wavesurfer.js';
  import { listen } from '@tauri-apps/api/event'; // Correct import for Tauri events

  let waveform;
  let waveformContainer;
  let samples = [];

  onMount(async () => {
    // Initialize WaveSurfer
    waveform = WaveSurfer.create({
      container: waveformContainer,
      waveColor: '#646cff',
      progressColor: '#4a4a4a',
      cursorColor: '#4a4a4a',
      height: 100,
      responsive: true,
    });

    // Listen for real-time audio samples from the backend
    const unlisten = await listen('audio_sample', (event) => {
      const sample = event.payload;
      console.log('Received sample:', sample); // Log received sample
      samples.push(sample);

      // Update the waveform every 100 samples
      if (samples.length >= 100) {
        waveform.load('', new Float32Array(samples)); // Update the waveform
        samples = []; // Clear the samples buffer
      }
    });

    // Cleanup the event listener when the component is destroyed
    return () => {
      unlisten(); // Stop listening for events
      waveform.destroy(); // Clean up WaveSurfer
    };
  });
</script>

<div bind:this={waveformContainer} style="width: 200px; height: 100px;"></div>
