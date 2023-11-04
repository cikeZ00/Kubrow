<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let greetMsg = ""

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let selection = "Greet";
    greetMsg = await invoke("select", { selection })
  }

  async function option1() {
    let selection = "Map";
    greetMsg = await invoke("select", { selection })

  }

  async function option2() {
    let selection = "Wiki";
    greetMsg = await invoke("select", { selection })

  }
</script>

<div>
  <ul class="menu">
    <li><a href="#" on:click={greet}>Greet</a></li>
    <li><a href="/map">Map</a></li>
    <li><a href="#" on:click={option2}>Wiki</a></li>
    <!-- Add more menu items here with their respective functions -->
  </ul>

  <p>{greetMsg}</p>
  <!-- You can display messages or content related to each menu option here -->
</div>