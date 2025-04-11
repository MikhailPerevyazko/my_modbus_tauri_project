<script lang="ts">
  import { onMount } from "svelte";
  import { ParameterManager } from "../../parameter_manager.js";

  let timeNow = $state("");
  let counter = $state(0);

  let manager = new ParameterManager();
  let tableData = $state(manager.getTableData());

  onMount(() => {
    const interval = setInterval(() => {
      manager.updateAll;
      tableData = manager.getTableData()
    }, 1000);

    return () => clearInterval(interval)
  });

  let ids = $state(manager.getTableData().ids);
  let values =  $state(manager.getTableData().values);
  
  let dataArr = $state({});
  // Создаем экспорт стрелочной функции для использование в других компонентах.
  export const update = () => {
    timeNow = Date();
    counter += 1;

    for (let index in ids) {
      let id = ids[index];
      let getValue = manager.getValueFromId(id);
      dataArr[id] = getValue;
      manager.setValue(id, getValue + 1);
    }
  };
</script>


<div>
  <input value={timeNow} class=" w-full font-serif" />
  <div>
    <input
      value="Updated: {counter} times"
      class="underline mt-2s w-full font-serif" 
    />
  </div>
  <div class="grid grid-cols-2">
    <div class="grid grid-cols-1 mt-8">
      {#each Object.entries(dataArr) as [id, val]}
        <div class="mb-2">
          <input type="text" value="{id} - {val}" />
        </div>
      {/each}
    </div>
    <div class="mt-4 overflow-auto">
      <table class="min-w-full divide-y divide-gray-300">
        <thead class="bg-grey-50">
          <tr>
            <th scope="col" class="px-6 py-3 text-center text-xs font-base text-gray-500 uppercase tracking-wider">ID</th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-base text-gray-500 uppercase tracking-wider">Value</th>
          </tr>
        </thead>
        {#each ids as id, index }
          <tbody class="bg-white divide-y divide-gray-300">
            <tr class="hover:bg-gray-50 transition-colors duration-150">
              <th class="px-0 py-4 whitespace-nowrap text-medium font-medium text-gray-900">{id}</th>
              <td class="px-4 py-4 whitespace-nowrap">
                <span class="px-2 inline-flex text-medium leading-5 font-semibold rounded-md bg-green-200 text-green-1000">{tableData.values[index]}</td>
            </tr>
          </tbody>
        {/each}
      </table>
    </div>
  </div>
</div>
