<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Svrollbar } from "svrollbar";
  import Range from "./Range.svelte";

  const tracks = [
    { id: 'master', name: 'Master Volume', type: 'master' },
    { id: 'white',  name: 'White Noise',   type: 'white' },
    { id: 'brown',  name: 'Brown Noise',   type: 'brown' },
    { id: 'pink',   name: 'Pink Noise',    type: 'pink' },
    { id: 'rain1',  name: 'Light Rain',    type: 'assets/111405.ogg' },
    { id: 'rain2',  name: 'Medium Rain',   type: 'assets/114354.ogg' },
    { id: 'rain3',  name: 'Heavy Rain',    type: 'assets/111154.ogg' },
  ];

  let trackStates = {};

  async function loadTrack(id) {
    if (tracks.find(t => t.id === id).type.startsWith('assets/')) { // ik its hacky
      await invoke('add_file_track', { id, filePath: tracks.find(t => t.id === id).type });
    }
    else {
      await invoke('add_noise_track', { id, noiseType: tracks.find(t => t.id === id).type });
    }

    trackStates[id] = { playing: false };
  }

  async function loadAllTracks() {
    for (let t of tracks) {
      if (t.id === 'master') {
        continue;
      }
      await loadTrack(t.id);
    }
  }

  // initialize all tracks before they're used
  // helps with some bugs trying to modify volumes for tracks that don't exist
  loadAllTracks();

  async function toggleTrack(id) {
    if (!trackStates[id]) {
      await loadTrack(id);
    }
    trackStates[id].playing = !trackStates[id].playing;
    await invoke(trackStates[id].playing ? 'resume_track' : 'pause_track', { id });
  }

  function updateVolume(id, value) {
    if (id === 'master') {
      invoke('set_master_volume', { id, volume: parseFloat(value / 100.0) });
    }
    else {
      invoke('set_volume', { id, volume: parseFloat(value / 100.0) });
    }
  }

  import logo from "$lib/assets/pluvio_banner_app.png"

  // Scrollbar binds for svrollbar
  export let viewport: Element
  export let contents: Element
</script>

<div class="background"></div>

<div class="container noselect">
  <img class="title nodrag" src={logo} alt="Pluvio">

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

  <div bind:this={viewport} class="tracks__viewport">
    <div bind:this={contents} class="tracks__scrollable">

      <!-- Individual Tracks -->
      <div class="tracks__container">
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

      <!-- Background color -->
      <div class="plates__container">
        <div class="scroll-cover scroll-cover-top"></div>
        <div class="scroll-cover scroll-cover-bottom"></div>
        {#each tracks.filter(t => t.type !== 'master') as track}
          <div class="plate {trackStates[track.id]?.playing ? 'active' : 'inactive'}">
            <div class="color bottom"></div>
            <div class="color top"></div>
          </div>
        {/each}
      </div>

    </div>
  </div>

  <!--<Svrollbar {viewport} {contents} />-->
</div>

<style>
  :root {
    overflow: none;
  }

  /* .noselect */
  :root,
  .container {
    -webkit-touch-callout: none; /* iOS Safari */
      -webkit-user-select: none; /* Safari */
       -khtml-user-select: none; /* Konqueror HTML */
         -moz-user-select: none; /* Old versions of Firefox */
          -ms-user-select: none; /* Internet Explorer/Edge */
              user-select: none;
  }

  .nodrag {
    pointer-events: none;
    user-drag: none;
    user-select: none;
    -moz-user-select: none;
    -webkit-user-drag: none;
    -webkit-user-select: none;
    -ms-user-select: none;
  }

  .master-volume {
    --track-focus: #1f1f1f;
    --track-highlight-bgcolor: #1f1f1f;
    --track-highlight-bg: linear-gradient(359deg, #1f1f1f, #6b6b6b);
    --thumb-holding-outline: rgba(255, 255, 255, 0.2);
    --tooltip-bgcolor: #1f1f1f;
    --tooltip-bg: linear-gradient(45deg, #6b6b6b, #1f1f1f);
  }
  .active .track-volume {
    --track-focus: #68b8fe;
    --track-highlight-bgcolor: #68b8fe;
    --track-highlight-bg: linear-gradient(90deg, #68b8fe, #2d65ff);
    --thumb-holding-outline: rgba(255, 255, 255, 0.2);
    --tooltip-bgcolor: #3879ff;
    --tooltip-bg: linear-gradient(45deg, #275cef, #5193ff);
  }
  .inactive .track-volume {
    --track-focus: #a0a0a0;
    --track-highlight-bgcolor: #a0a0a0;
    --track-highlight-bg: #a0a0a0;
    --thumb-holding-outline: transparent;
    --tooltip-bgcolor: #a0a0a0;
    --tooltip-bg: #a0a0a0;
    --track-bgcolor: #a0a0a0;
    --thumb-bgcolor: #a0a0a0;
  }
  .inactive .track-label {
    color: #a0a0a0;
  }

  .background {
    position: fixed;
    z-index: -1;
    width: 100%;
    height: 100%;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(45deg, #002041 0%, #002b41 30%, #400f41 80%, #390d49 100%);
  }

  .container {
    font-size: 12px;
    font-family: sans-serif;
    width: 380px;
    margin: 0 auto;
    color: white;
    overflow: none;
  }

  .tracks__viewport {
    overflow-y: scroll;
    height: 240px;
    border-radius: 28px;
    padding: 16px;
    box-shadow: inset 0 0 22px 0 rgba(0, 0, 0, 0.7);
    background: linear-gradient(0deg,
      #10108060 0%,
      #4fffdc08 30%,
      #10106007 60%,
      #50105040 100%
    );
  }

  .tracks__scrollable {
    position: relative;
    z-index: 2;
  }

  .plates__container,
  .tracks__container {
    padding: 2px 0;
  }

  .scroll-cover {
    width: 380px;
    height: 110px;
    position: fixed;
    z-index: 2;
    object-fit: contain;
    background-repeat: no-repeat;
    margin-left: -16px;
  }
  .scroll-cover-top {
    background-image: url("$lib/assets/fade/fade_e4_white_top.svg");
    margin-top: -2px;
  }
  .scroll-cover-bottom {
    background-image: url("$lib/assets/fade/fade_e4_white_bottom.svg");
    margin-top: 160.75px;
  }

  .plates__container {
    position: absolute;
    top: 0;
    z-index: -1;
    margin-top: -16px;
  }

  .plate > .color {
    box-shadow: 5px 5px 8px 0px #00000060;
    width: 349px;
    position: relative;
    -webkit-transition: opacity 700ms cubic-bezier(0, 0.5, 0, 1 );
    -moz-transition: opacity 700ms cubic-bezier(0, 0.5, 0, 1);
    transition: opacity 700ms cubic-bezier(0, 0.5, 0, 1);
  }

  .plate > .color.bottom {
    background: linear-gradient(45deg, #002143 0%, #290038 100%);
  }
  .plate > .color.top {
    background: linear-gradient(45deg, #0e4c8c 0%, #751598 100%);
    position: absolute;
    z-index: 1;
    margin-top: -50px;
    opacity: 0%;
  }

  .active.plate > .color.top {
    opacity: 100%;
  }

  .track,
  .plate > .color {
    margin-top: 16px;
    border-radius: 20px;
    height: 50px;
  }

  .track {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    width: 321px;
    cursor: pointer;
  }

  .track:first-of-type,
  .plate:first-of-type {
    margin-top: 0;
  }

  .track.master {
    font-size: 16px;
    background: transparent;
    width: auto;
    margin: 8px 0 16px;
  }

  .track-label {
    font-weight: bold;
    padding-left: 5px;
    cursor: pointer;
  }

  .track.master,
  .track.master > .track-label {
    cursor: initial;
  }

  .volume-container {
    width: 215px;
  }

  .track.master > .volume-container {
    width: 415px;
  }

  .title {
    font-size: 30px;
    margin-left: 20px;
    margin-bottom: 0;
    margin-top: 30px;
    object-fit: contain;
    width: 340px;
  }
</style>
