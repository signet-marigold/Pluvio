<script lang="ts">

  import { invoke } from "@tauri-apps/api/core";
  import { Svrollbar } from "svrollbar";
  import Range from "./Range-scalable.svelte";

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

  <div class="wrapper">
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
        hideAfter={1750} />
    </div>
  </div>
</div>

<style lang="sass">
@use 'sass:math'

$scale: 440

$bpx: math.div(100vh, $scale)

$track-margin: $bpx*20
$track-padding: $bpx*16
$track-height: $bpx*44
$track-view-width: $bpx*380
$track-view-height: $bpx*258
$track-extra-space-top: $bpx*5
$track-extra-space-bottom: $bpx*5
$track-label-width: $bpx*110
$track-label-padding: $bpx*5
$title-margin: $bpx*20

$track-view-sc-height: $bpx*110
$track-width:
			$track-view-width -
			$track-padding * 2 -
			$track-margin * 2
$track-plate-width:
			$track-view-width -
			$track-margin * 2
$track-view-sc-offset-top:
			$track-view-height -
			$track-view-sc-height +
			$track-margin * 2 -
			$track-extra-space-top
$track-view-sc-offset-bottom: (
			$track-view-sc-height -
			$track-margin -
			$track-extra-space-bottom) * -1
$track-volume-width: $track-width - $track-label-width

:root,
.main
	overflow: hidden
	-webkit-touch-callout: none
	-webkit-user-select: none
	-khtml-user-select: none
	-moz-user-select: none
	-ms-user-select: none
	user-select: none

.nodrag
	pointer-events: none
	user-drag: none
	user-select: none
	-moz-user-select: none
	-webkit-user-drag: none
	-webkit-user-select: none
	-ms-user-select: none

.master-volume
	--track-focus: #831c79
	--track-highlight-bgcolor: #831c79
	--track-highlight-bg: linear-gradient(90deg, #491c83, #831c79, #cd273d)
	--thumb-holding-outline: rgba(255, 255, 255, 0.2)
	--tooltip-bgcolor: #831c79
	--tooltip-bg: linear-gradient(230deg, #cd273d, #831c79)

.active
	.track-volume
		--track-focus: #68b8fe
		--track-highlight-bgcolor: #68b8fe
		--track-highlight-bg: linear-gradient(90deg, #68b8fe, #2d65ff)
		--thumb-holding-outline: rgba(255, 255, 255, 0.2)
		--tooltip-bgcolor: #3879ff
		--tooltip-bg: linear-gradient(45deg, #275cef, #5193ff)
	&.plate
		& > .color
			&.top
				opacity: 100%

.inactive
	$inactive-base-color: #7b7b7b
	.track-volume
		--track-focus: #{$inactive-base-color}
		--track-highlight-bgcolor: #{$inactive-base-color}
		--track-highlight-bg: #{$inactive-base-color}
		--thumb-holding-outline: transparent
		--tooltip-bgcolor: #{$inactive-base-color}
		--tooltip-bg: #{$inactive-base-color}
		--track-bgcolor: #{$inactive-base-color}
		--thumb-bgcolor: #{$inactive-base-color}
	.track-label
		color: $inactive-base-color

.background
	position: fixed
	z-index: -1
	width: 100%
	height: 100%
	top: 0
	bottom: 0
	left: 0
	right: 0
	background: linear-gradient(45deg, #002041 0%, #002b41 30%, #400f41 80%, #390d49 100%)

.main
	font-size: $bpx*12
	font-family: Quicksand, sans-serif
	font-weight: bold
	width: $track-view-width
	margin: 0 auto
	color: white
	--svrollbar-track-width: #{$bpx*12}
	--svrollbar-track-background: #111111
	--svrollbar-track-opacity: 0
	--svrollbar-thumb-width: #{$bpx*6}
	--svrollbar-thumb-background: #724077
	--svrollbar-thumb-opacity: 1
	--svrollbar-thumb-shadow: #{inset $bpx $bpx $bpx 0 #c998ce, inset $bpx*-1 $bpx*-1 $bpx*1.5 0 #261428}

.wrapper
	position: relative

.title
	margin: 0 $title-margin
	width: $track-view-width - $title-margin * 2
	margin-top: $bpx*24
	object-fit: contain

.tracks__viewport
	height: $track-view-height
	border-radius: $bpx*28
	padding: $track-margin
	box-shadow: inset 0 0 $bpx*22 0 rgba(0, 0, 0, 0.7)
	background: linear-gradient(0deg, #12000090, #00120b70)
	overflow-x: hidden
	overflow-y: scroll
	-ms-overflow-style: none
	scrollbar-width: none
	&::-webkit-scrollbar
		display: none

.tracks__scrollable
	position: relative
	z-index: 2

.plates__container,
.tracks__container
	padding: $track-extra-space-top 0 $track-extra-space-bottom

.scroll-cover,
.scroll-bookend
	width: $track-view-width
	height: $track-view-sc-height
	object-fit: contain
	background-repeat: no-repeat
	margin-left: $track-margin * -1

.scroll-cover
	position: fixed
	z-index: 2

.scroll-cover-top
	background-image: url("$lib/assets/fade/fade-top.svg")
	margin-top: $track-extra-space-top * -1

.scroll-cover-bottom
	background-image: url("$lib/assets/fade/fade-bottom.svg")
	margin-top: $track-view-sc-offset-top

.scroll-bookend
	position: absolute
	z-index: 3

.scroll-bookend-top
	background-image: url("$lib/assets/fade/fade_e4_white_top.svg")
	margin-top: $track-extra-space-top * -1

.scroll-bookend-bottom
	background-image: url("$lib/assets/fade/fade_e4_white_bottom.svg")
	margin-top: $track-view-sc-offset-bottom

.plates__container
	position: absolute
	top: 0
	z-index: -1
	margin-top: $track-margin * -1

.plate
	& > .color
		box-shadow: $bpx*5 $bpx*5 $bpx*8 0 #00000060, inset 0 $bpx*2 $bpx*2 0 rgba(255, 255, 255, 0.1)
		width: $track-plate-width
		position: relative
		-webkit-transition: opacity 700ms cubic-bezier(0, 0.5, 0, 1)
		-moz-transition: opacity 700ms cubic-bezier(0, 0.5, 0, 1)
		transition: opacity 700ms cubic-bezier(0, 0.5, 0, 1)
		&.bottom
			background: linear-gradient(45deg, #002041, #400f41)
		&.top
			background: linear-gradient(45deg, #042580, #72185e)
			position: absolute
			z-index: 1
			margin-top: $track-height * -1
			opacity: 0%

.track,
.plate > .color
	margin-top: $track-margin
	border-radius: $bpx*18
	height: $track-height

.track
	display: flex
	align-items: center
	justify-content: space-between
	padding: 0 $track-padding
	width: $track-width
	cursor: pointer
	&.master
		background: transparent
		width: auto
		margin: $bpx*8 0 $bpx*12
		& > .volume-container
			width: 100%

.tracks__viewport
	.track:first-of-type,
	.plate:first-of-type
		margin-top: 0

.track-label
	font-weight: 600
	padding-left: $track-label-padding
	cursor: pointer

.volume-container
	width: $track-volume-width

.track.master,
.track.master > .track-label
	font-size: $bpx*14
	line-height: 1.1
	cursor: default

</style>
