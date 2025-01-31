<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { ParameterManager } from "../../parameter_manager.mjs";

  let timeNow = $state("");
  let counter = $state(0);

  const pIDs = ["id1", "id2", "id3"];
  let parameterManager = new ParameterManager();
  let dataArr: any = $state({});

  // Создаем экспорт стрелочной функции для использование в других компонентах.
  export const update = () => {
    timeNow = Date();
    counter += 1;

    for (let index in pIDs) {
      let id = pIDs[index];
      let get_value = parameterManager.getValue(id);
      dataArr[id] = get_value;
      parameterManager.setValue(id, get_value + 1);
    }
  };
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
    <div class="grid grid-cols-1 mt-4">
      {#each Object.entries(dataArr) as [id, val]}
        <div class="mb-2">
          <input type="text" value="{id} - {val}" />
        </div>
      {/each}
    </div>
  </div>
  <div class="ml-2 mb-4">
    <button class="bg-blue-600 hover:bg-yellow-400 px-5 py-3 rounded-lg"
      >Применить</button
    >
  </div>
  <div class="grid grid-cols-5">
    <div>
      <h1 class="underline">Name:</h1>
      <div></div>
    </div>
    <div>
      <h1 class="underline">Unit ID:</h1>
    </div>
    <div>
      <h1 class="underline">Ptype:</h1>
    </div>
    <div>
      <h1 class="underline">Start address:</h1>
    </div>
    <div>
      <h1 class="underline">Mstorage:</h1>
    </div>
  </div>
</div>
