<script lang="ts">
  import { Tabs, TabItem } from "flowbite-svelte";
  import Configs from "./Configs.svelte";

  // Импортируем компонент с экспортной функцией, создаем переменную.
  import ModbusView from "./ModbusView.svelte";
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
    <TabItem open title=" Modbus View">
      <ModbusView bind:this={modbusView} />
    </TabItem>
    <TabItem open title="Configs">
      <Configs />
    </TabItem>
  </Tabs>
</div>
