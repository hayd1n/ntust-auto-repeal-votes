<script lang="ts">
  import "../app.css";
  let { children } = $props();
  import { Button } from "$lib/components/ui/button";
  import { AUTHOR_GITHUB } from "$lib/manifest";
  import { open } from "@tauri-apps/plugin-shell";
</script>

<main
  class="flex flex-col justify-between items-center py-10 px-20 text-center w-screen h-screen"
>
  <div class="flex flex-col items-center gap-2">
    <img src="/ntust.png" alt="NTUST Logo" class="w-20" />
    <h1 class="font-bold text-xl">NTUST Auto Repeal Votes</h1>
    <p class="text-sm">
      I just want to see final grades, I don't care about course evaluations.<br
      />This app is designed for you.
    </p>
  </div>
  <div
    class="my-10 h-full w-8/12 flex flex-col justify-center gap-2 overflow-hidden"
  >
    {@render children()}
  </div>
  <footer class="text-xs text-gray-500 font-mono">
    <p>
      Made with ❤️ by <button
        class="cursor-pointer hover:underline"
        onclick={() => open(AUTHOR_GITHUB)}>Hayden Chang</button
      >
    </p>
  </footer>
</main>
