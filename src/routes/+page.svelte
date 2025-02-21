<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  import Range from "./Range.svelte"

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
      invoke('set_master_volume', { id, volume: parseFloat(value / 100.0) });
    }
    else {
      invoke('set_volume', { id, volume: parseFloat(value / 100.0) });
    }
  }
</script>

<div class="background"></div>

<div class="container noselect">
  <h1 class="title">Pluvio</h1>

  <!-- Master Volume -->
  <div class="track master">
    <label class="track-label">Master Volume</label>
    <div class="volume-container master-volume">
      <Range
        min={0} max={100}
        step={1} value={50}
        on:change={(e) => updateVolume('master', e.detail.value)}
      />
    </div>
  </div>

  <!-- Individual Tracks -->
  {#each tracks.filter(t => t.type !== 'master') as track}
    <div class="track {trackStates[track.id]?.playing ? 'active' : 'inactive'}" on:click={() => toggleTrack(track.id)}>
      <label class="track-label">{track.name}</label>
      <div class="volume-container track-volume">
        <Range
          min={0} max={100}
          step={1} value={25}
          on:change={(e) => updateVolume(track.id, e.detail.value)}
        />
      </div>
    </div>
  {/each}
</div>

<style>
  .noselect {
    -webkit-touch-callout: none; /* iOS Safari */
      -webkit-user-select: none; /* Safari */
      -khtml-user-select: none; /* Konqueror HTML */
        -moz-user-select: none; /* Old versions of Firefox */
          -ms-user-select: none; /* Internet Explorer/Edge */
              user-select: none;
  }

  .master-volume {
    --track-focus: #1f1f1f;
    --track-highlight-bgcolor: #1f1f1f;
    --track-highlight-bg: linear-gradient(359deg, #1f1f1f, #6b6b6b);
    --thumb-holding-outline: rgba(255, 255, 255, 0.2);
    --tooltip-bgcolor: #1f1f1f;
    --tooltip-bg: linear-gradient(45deg, #6b6b6b, #1f1f1f);
  }
  .track-volume {
    --track-focus: #68b8fe;
    --track-highlight-bgcolor: #68b8fe;
    --track-highlight-bg: linear-gradient(90deg, #68b8fe, #2d65ff);
    --thumb-holding-outline: rgba(255, 255, 255, 0.2);
    --tooltip-bgcolor: #3879ff;
    --tooltip-bg: linear-gradient(45deg, #275cef, #5193ff);
  }
  .inactive {
    --track-focus: #717171;
    --track-highlight-bgcolor: #717171;
    --track-highlight-bg: #717171;
    --thumb-holding-outline: transparent;
    --tooltip-bgcolor: #717171;
    --tooltip-bg: #717171;
    --track-bgcolor: #717171;
    --thumb-bgcolor: #717171;
  }
  .inactive .track-label {
    color: #717171;
  }

  .active.track {
    background: linear-gradient(45deg, #0e5daf, #7a1a7c);
  }

  .background {
    position: absolute;
    z-index: -1;
    width: 100%;
    height: 100%;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(45deg, #001831, #3d0d3e);
  }

  .container {
    width: 400px;
    margin: 0 auto;
    color: white;
  }

  .track {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin: 16px 0;
    padding: 14px;
    background: linear-gradient(45deg, #001831, #3d0d3e);
    border-radius: 24px;
    cursor: pointer;
  }

  .track.master {
    background: transparent;
    font-size: 1.2em;

  }

  .track-label {
    width: 150px;
    font-weight: bold;
    padding-left: 5px;
    cursor: pointer;
  }

  .track.master,
  .track.master .track-label {
    cursor: initial;
  }

  .volume-container {
    width: 245px;
  }

  .title {
    margin-left: 15px;
    font-size: 2.2em;
    margin-bottom: 0;
    margin-top: 28px;
  }

</style>
