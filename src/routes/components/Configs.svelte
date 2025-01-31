<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
</script>

<div>
  <div class="ml-2 mb-4">
    <button class="bg-blue-600 hover:bg-yellow-400 px-5 py-3 rounded-lg">
      Применить
    </button>
  </div>

  <div class="grid grid-cols-3">
    <div>
      <h1 class="underline">Protocol type:</h1>
    </div>
    <div>
      <h1 class="underline">Connections:</h1>
    </div>
    <div>
      <h1 class="underline">Channel ID:</h1>
    </div>
  </div>
</div>
