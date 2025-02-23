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

  // Binds for svrollbar
  export let viewport: Element
  export let contents: Element
</script>

<div class="background"></div>

<div class="main">
<div class="wrapper noselect">
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
        <div class="scroll-bookend scroll-bookend-top"></div>
        {#each tracks.filter(t => t.type !== 'master') as track}
          <div class="plate {trackStates[track.id]?.playing ? 'active' : 'inactive'}">
            <div class="color bottom"></div>
            <div class="color top"></div>
          </div>
        {/each}
        <div class="scroll-bookend scroll-bookend-bottom"></div>
      </div>

    </div>
  </div>

  <Svrollbar
      {viewport} {contents}
      margin={{top:160,bottom:-125}}
      hideAfter={1750} />
</div>
</div>

<style>
  :root,
  .wrapper {
    overflow: hidden;
  }

  /* .noselect */
  :root,
  .wrapper {
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
    --track-focus: #831c79;
    --track-highlight-bgcolor: #831c79;
    --track-highlight-bg: linear-gradient(90deg, #491c83, #831c79, #cd273d);
    --thumb-holding-outline: rgba(255, 255, 255, 0.2);
    --tooltip-bgcolor: #831c79;
    --tooltip-bg: linear-gradient(230deg, #cd273d, #831c79);
  }
  .active .track-volume {
    --track-focus: #68b8fe;
    --track-highlight-bgcolor: #68b8fe;
    --track-highlight-bg: linear-gradient(90deg, #68b8fe, #2d65ff);
    --thumb-holding-outline: rgba(255, 255, 255, 0.2);
    --tooltip-bgcolor: #3879ff;
    --tooltip-bg: linear-gradient(45deg, #275cef, #5193ff);
  }
  .inactive {--inactive-base-color: #7b7b7b}
  .inactive .track-volume {
    --track-focus: var(--inactive-base-color);
    --track-highlight-bgcolor: var(--inactive-base-color);
    --track-highlight-bg: var(--inactive-base-color);
    --thumb-holding-outline: transparent;
    --tooltip-bgcolor: var(--inactive-base-color);
    --tooltip-bg: var(--inactive-base-color);
    --track-bgcolor: var(--inactive-base-color);
    --thumb-bgcolor: var(--inactive-base-color);
  }
  .inactive .track-label {color: var(--inactive-base-color)}

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

  .main {
  }

  .wrapper {
    --track-margin: 20px;
    --track-padding: 16px;
    --track-height: 44px;
    --track-view-width: 380px;
    --track-view-height: 258px;
    --track-extra-space-top: 5px;
    --track-extra-space-bottom: 5px;
    --track-label-width: 110px;
    --track-label-padding: 5px;
    --title-margin: 20px;

    font-size: 12px;
    font-family: sans-serif;
    width: var(--track-view-width);
    position: relative;
    margin: 0 auto;
    color: white;

    /* Svrollbar settings */
    --svrollbar-track-width: 6px;
    --svrollbar-track-background: #111111;
    --svrollbar-track-opacity: 0;

    --svrollbar-thumb-width: 6px;
    --svrollbar-thumb-background: #724077;
    --svrollbar-thumb-opacity: 1;
    --svrollbar-thumb-shadow: inset 1px 1px 1px 0 #c998ce, inset -1px -1px 1.5px 0 #261428;
  }

  .title {
    margin: 0 var(--title-margin);
    width: calc(var(--track-view-width) -
        var(--title-margin) * 2);
    font-size: 30px;
    margin-top: 24px;
    object-fit: contain;
  }

  .tracks__viewport {
    --track-view-sc-height: 110px;
    --track-width: calc(
        var(--track-view-width) -
        var(--track-padding) * 2 -
        var(--track-margin) * 2);
    --track-plate-width: calc(
        var(--track-view-width) -
        var(--track-margin) * 2);
    --track-view-sc-offset-top: calc(
        var(--track-view-height) -
        var(--track-view-sc-height) +
        var(--track-margin) * 2 -
        var(--track-extra-space-top));
    --track-view-sc-offset-bottom: calc((
        var(--track-view-sc-height) -
        var(--track-margin) -
        var(--track-extra-space-bottom)) * -1);
    --track-volume-width: calc(var(--track-width) - var(--track-label-width));

    height: var(--track-view-height);
    border-radius: 28px;
    padding: var(--track-margin);
    box-shadow: inset 0 0 22px 0 rgba(0, 0, 0, 0.7);
    background: linear-gradient(0deg, #12000090, #00120b70);

    overflow-x: hidden;
    overflow-y: scroll;

    /* hide scrollbar */
    -ms-overflow-style: none;
    scrollbar-width: none;
  }

  .tracks__viewport::-webkit-scrollbar {
    /* hide scrollbar */
    display: none;
  }

  .tracks__scrollable {
    position: relative;
    z-index: 2;
  }

  .plates__container,
  .tracks__container {
    padding: var(--track-extra-space-top) 0 var(--track-extra-space-bottom);
  }

  .scroll-cover,
  .scroll-bookend {
    /*
      image is 380px wide
      edge curve is not preserved if stretched
      new image is required if scrollable width changes
    */
    width: var(--track-view-width);
    height: var(--track-view-sc-height);
    object-fit: contain;
    background-repeat: no-repeat;
    margin-left: calc(var(--track-margin) * -1);
  }

  .scroll-cover {
    position: fixed;
    z-index: 2;
  }
  .scroll-cover-top {
    background-image: url("$lib/assets/fade/fade-top.svg");
    margin-top: calc(var(--track-extra-space-top) * -1);
  }
  .scroll-cover-bottom {
    background-image: url("$lib/assets/fade/fade-bottom.svg");
    margin-top: var(--track-view-sc-offset-top);
  }

  .scroll-bookend {
    position: absolute;
    z-index: 3;
  }
  .scroll-bookend-top {
    background-image: url("$lib/assets/fade/fade_e4_white_top.svg");
    margin-top: calc(var(--track-extra-space-top) * -1);
  }
  .scroll-bookend-bottom {
    background-image: url("$lib/assets/fade/fade_e4_white_bottom.svg");
    margin-top: var(--track-view-sc-offset-bottom);
  }

  .plates__container {
    position: absolute;
    top: 0;
    z-index: -1;
    margin-top: calc(var(--track-margin) * -1);
  }

  .plate > .color {
    box-shadow: 5px 5px 8px 0px #00000060,
        inset 0px 2px 2px 0 rgba(255, 255, 255, 0.1);
    width: var(--track-plate-width);
    position: relative;
    -webkit-transition: opacity 700ms cubic-bezier(0, 0.5, 0, 1 );
    -moz-transition: opacity 700ms cubic-bezier(0, 0.5, 0, 1);
    transition: opacity 700ms cubic-bezier(0, 0.5, 0, 1);
  }

  .plate > .color.bottom {
    background: linear-gradient(45deg, #002041, #400f41);
  }
  .plate > .color.top {
    background: linear-gradient(45deg, #042580, #72185e);
    position: absolute;
    z-index: 1;
    margin-top: calc(var(--track-height) * -1);
    opacity: 0%;
  }

  .active.plate > .color.top {
    opacity: 100%;
  }

  .track,
  .plate > .color {
    margin-top: var(--track-margin);
    border-radius: 18px;
    height: var(--track-height);
  }

  .track {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--track-padding);
    width: var(--track-width);
    cursor: pointer;
  }

  .track:first-of-type,
  .plate:first-of-type {
    margin-top: 0;
  }

  .track-label {
    font-weight: bold;
    padding-left: var(--track-label-padding);
    cursor: pointer;
  }

  .volume-container {
    width: var(--track-volume-width);
  }

  .track.master {
    font-size: 16px;
    background: transparent;
    width: auto;
    margin: 8px 0 16px;
  }

  .track.master > .volume-container {
    width: 100%;
  }

  .track.master,
  .track.master > .track-label {
    cursor: initial;
  }
</style>
