<script lang="ts">
  import TheTabs from "./components/TheTabs.svelte";
  import { onMount } from "svelte";

  let comp: any;
  onMount(() => {
    const interval = setInterval(() => {
      comp.update();
    }, 1000);
    return () => {
      clearInterval(interval);
    };
  });
</script>

<main>
  <div class="container">
    <TheTabs bind:this={comp} />
  </div>
</main>
