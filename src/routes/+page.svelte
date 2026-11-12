<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="w-full h-full flex flex-col justify-center">
    <div class="m-auto">
        <button class="btn btn-primary">New</button>
        <button class="btn btn-primary">Open</button>
    </div>
</div>
