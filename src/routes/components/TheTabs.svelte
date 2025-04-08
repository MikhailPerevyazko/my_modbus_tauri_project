<script lang="ts">
  import { Tabs, TabItem } from "flowbite-svelte";
  import Tab2 from "./Tab2.svelte";
  import Tab3 from "./Tab3.svelte";
  // Импортируем компонент с экспортной функцией, создаем переменную.
  import Tab1 from "./Tab1.svelte";
  let modbusView: any;

  // Вызов экспорт функции.
  export const update = () => {
    if (modbusView != null) {
      modbusView.update();
    }
  };

</script>

<div class="container">
  <Tabs>
    <TabItem open title=" Tab1">
      <Tab1 bind:this={modbusView} />
    </TabItem>

    <TabItem open title="Tab2">
      <Tab2 />
    </TabItem>

    <TabItem open title="Tab3">
      <Tab3 />
    </TabItem>
  </Tabs>
</div>
