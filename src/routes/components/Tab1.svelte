<script lang="js">
  import { ParameterManager } from "../../parameter_manager.js";

  let timeNow = $state("");
  let counter = $state(0);

  let parameterManager = new ParameterManager();

  let ids = $state(parameterManager.getAllIdes());
  let values = $state(parameterManager.getAllValues());

  let dataArr = $state({});

  // Создаем экспорт стрелочной функции для использование в других компонентах.
  export const update = () => {
    timeNow = Date();
    counter += 1;

    for (let index in ids) {
      let id = ids[index];
      let get_value = parameterManager.getVal(id);
      dataArr[id] = get_value;
      parameterManager.setVal(id, get_value + 1);
    }
  };

  function clear() {
    ids = []
    values = []
  }
</script>

<div>
  <div>
    <input value={timeNow} class="border border-amber-700 w-full" />
    <div>
      <input
        value="Updated {counter} times"
        class="underline border border-amber-700 mt-2 w-full"
      />
    </div>
    <div class="grid grid-cols-2">
      <div class="grid grid-cols-1 mt-4">
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
                  <span class="px-2 inline-flex text-medium leading-5 font-semibold rounded-md bg-green-200 text-green-1000">{values[index]}</td>
              </tr>
            </tbody>
          {/each}
        </table>
      </div>
    </div>
  </div>
</div>
<div>
  <button class="bg-black h-10 text-white text-sm rounded-lg" onclick={clear}>Очистить</button>
</div>
