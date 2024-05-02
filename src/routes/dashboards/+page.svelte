<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type { PageData } from "./$types";

  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import * as Card from "$lib/components/ui/card";
  import DashboardCreateModal from "./dashboardCreateModal.svelte";

  import { createScrollArea, melt } from "@melt-ui/svelte";

  import { keyed } from "$lib/keyed";
  import type { Dashboard, Controller, Page, DataSource } from "$lib/types";
  import { dashboardSelected, dashboardStore } from "$lib/stores";
  import { taurpc } from "$lib/ipc";
  import { FolderOpenDot, X, Trash2,FolderPlus, FolderOpen } from "lucide-svelte";

  export let data: PageData;

  const call_taurRPC = async () => {
    const str_res = await taurpc.api.hello_worldorini();
    console.log(str_res);
    const dBoards = await taurpc.api.get_dashboards();
    console.log(dBoards);
  };

  const {
    elements: {
      root,
      content,
      viewport,
      corner,
      scrollbarY,
      thumbY,
      thumbX,
      scrollbarX,
    },
  } = createScrollArea({
    type: "auto",
    dir: "ltr",
  });

  export let openDialog = false;

  export const dBoardName = keyed(dashboardStore, "name");
  
  onMount(async () => {
    const dBoards = await taurpc.api.get_dashboards();
    console.log(dBoards);
  });
  export const dashboardProjMock: Dashboard[] = [
    {
        name: "test dashboard 1",
        description: "this is a description",
        id: "",
        data_sources: [],
        controllers: [],
        visualizations: [],
        pages: [],
        file_path: ""
    },
    {
        name: "test dashboard 2",
        description: "this is a description",
        id: "",
        data_sources: [],
        controllers: [],
        visualizations: [],
        pages: [],
        file_path: ""
    },
    {
        name: "test dashboard 3",
        description: "this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description ",
        id: "",
        data_sources: [],
        controllers: [],
        visualizations: [],
        pages: [],
        file_path: ""
    },
    {
        name: "test dashboard 4",
        description: "this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description ",
        id: "",
        data_sources: [],
        controllers: [],
        visualizations: [],
        pages: [],
        file_path: ""
    },
    {
        name: "test dashboard 4",
        description: "this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description ",
        id: "",
        data_sources: [],
        controllers: [],
        visualizations: [],
        pages: [],
        file_path: ""
    },
    {
        name: "test dashboard 4",
        description: "this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description ",
        id: "",
        data_sources: [],
        controllers: [],
        visualizations: [],
        pages: [],
        file_path: ""
    },
    {
        name: "test dashboard 4",
        description: "this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description ",
        id: "",
        data_sources: [],
        controllers: [],
        visualizations: [],
        pages: [],
        file_path: ""
    },
    {
        name: "test dashboard 4",
        description: "this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description ",
        id: "",
        data_sources: [],
        controllers: [],
        visualizations: [],
        pages: [],
        file_path: ""
    },
    {
        name: "test dashboard 4",
        description: "this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description this is a description this is a descriptionthis is a descriptionthis is a description ",
        id: "",
        data_sources: [],
        controllers: [],
        visualizations: [],
        pages: [],
        file_path: ""
    },
  ];
</script>

<section>
  <div
    class="bg-muted/40 min-h-[calc(100vh_-_theme(spacing.16))] select-none"
  >
    <p>testing</p>
    <Button on:click={call_taurRPC}>Test tauri RPC</Button>
    <Input bind:value={$dBoardName} />
    <p>{$dBoardName}</p>
    <div class="grid grid-cols-6 gap-4">
      <div class="col-span-4 px-2">
        <div
          use:melt={$root}
          class="h-full overflow-hidden border"
        >
          <div
            use:melt={$viewport}
            class="h-full w-full rounded-[inherit]"
          >
          <div use:melt={$content}>
            {#each dashboardProjMock as board}
            <div class="grid grid-cols-3 border">
              <div class="col-span-1  h-12 flex items-center justify-start font-semibold">
                {board.name}
              </div>
              <div class="col-span-2 h-12 flex items-center justify-end">
                <Button variant="outline" size="icon">
                  <FolderOpenDot />
                </Button>
                <Button variant="secondary" size="icon">
                  <X />
                </Button>
                <Button variant="destructive" size="icon">
                  <Trash2 />
                </Button>
              </div>
              <div class="col-span-3  h-12 flex items-center justify-start font-light overflow-hidden whitespace-nowrap truncate">
                <p class="truncate">{board.description}</p>
              </div>
            </div>
            {/each}

          </div>
          </div>
          <div
            use:melt={$scrollbarY}
            class="flex h-full w-2.5 touch-none select-none border-l border-l-transparent bg-neutral-300/10 p-px transition-colors"
          >
            <div
              use:melt={$thumbY}
              class="relative flex-1 rounded-full bg-neutral-300/50"
            />
          </div>
          <div use:melt={$corner} />
        </div>
      </div>
      <div class="col-span-2">
        <!-- svelte-ignore missing-declaration -->
        <div class="text-white bg-slate-800 hover:bg-slate-500 flex justify-start items-center px-4 py-2 my-2 text-2xl font-bold"><FolderOpen class="mr-2 text-yellow-700"/> Open existing dashboard</div>
        <DashboardCreateModal/>
      </div>
    </div>

  </div>
</section>
