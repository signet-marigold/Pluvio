<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const tracks = [
    { id: 'master', name: 'Master Volume', type: 'master' },
    { id: 'white',  name: 'White Noise',   type: 'white' },
    { id: 'brown',  name: 'Brown Noise',   type: 'brown' },
    { id: 'pink',   name: 'Pink Noise',    type: 'pink' },
    { id: 'rain1',  name: 'Light Rain',    type: 'assets/111405.wav' },
    { id: 'rain2',  name: 'Medium Rain',   type: 'assets/114354.wav' },
    { id: 'rain3',  name: 'Heavy Rain',    type: 'assets/111154.wav' },
  ];

  let trackStates = {};

  async function toggleTrack(id) {
    if (!trackStates[id]) {
      if (tracks.find(t => t.id === id).type.startsWith('assets/')) { // ik its hacky
        await invoke('add_file_track', { id, filePath: tracks.find(t => t.id === id).type });
      }
      else {
        await invoke('add_noise_track', { id, noiseType: tracks.find(t => t.id === id).type });
      }

      trackStates[id] = { playing: false };
    }
    trackStates[id].playing = !trackStates[id].playing;
    await invoke(trackStates[id].playing ? 'resume_track' : 'pause_track', { id });
  }

  function updateVolume(id, value) {
    if (id == 'master') {
      invoke('set_master_volume', { id, volume: parseFloat(value) });
    }
    else {
      invoke('set_volume', { id, volume: parseFloat(value) });
    }
  }
</script>

<div class="container">
  <h1>Noise Machine</h1>

  <!-- Master Volume -->
  <div class="track master">
    <label>Master Volume</label>
    <input
      type="range"
      min="0" max="1" step="0.01"
      on:input={(e) => updateVolume('master', e.target.value)}
      value="0.5"
    />
  </div>

  <!-- Individual Tracks -->
  {#each tracks.filter(t => t.type !== 'master') as track}
    <div class="track">
      <div class="controls">
        <button on:click={() => toggleTrack(track.id)}>
          {trackStates[track.id]?.playing ? '⏸' : '▶'}
        </button>
        <label>{track.name}</label>
      </div>
      <input
        type="range"
        min="0" max="1" step="0.01"
        on:input={(e) => updateVolume(track.id, e.target.value)}
        value="0.25"
      />
    </div>
  {/each}
</div>

<style>
  .container {
    padding: 2rem;
    max-width: 600px;
    margin: 0 auto;
  }

  .track {
    display: flex;
    align-items: center;
    margin: 1rem 0;
    padding: 1rem;
    background: #f5f5f5;
    border-radius: 8px;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 150px;
  }

  button {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    background: #646cff;
    color: white;
    cursor: pointer;
  }

  input[type="range"] {
    flex: 1;
    margin-left: auto;
  }
</style>
