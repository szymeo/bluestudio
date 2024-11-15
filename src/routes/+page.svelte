<script lang="ts">
  import "./global.css";
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="w-full h-full text-white">
  <div data-tauri-drag-region="true" class="draggable h-6 w-full"></div>
</main>
