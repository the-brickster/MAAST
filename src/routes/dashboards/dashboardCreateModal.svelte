<script lang="ts">
  import { goto } from "$app/navigation";
  import { melt, createDialog, createSync } from "@melt-ui/svelte";
  import { X, FolderPlus } from "lucide-svelte";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";


  import {open as open_t} from "@tauri-apps/api/dialog";
  import {documentDir} from "@tauri-apps/api/path"

  import { dashboardSelected, dashboardStore } from "$lib/stores";

  const {
    elements: {
      trigger,
      portalled,
      overlay,
      content,
      title,
      description,
      close,
    },
    states: { open },
  } = createDialog();

  export let dashboardName:string = "";
  export let dashboardDesc: string = "";
  export let dashboardLoc: string = "";

  const isDashboardSelected = dashboardSelected;
  const currDashboard = dashboardStore;

  const createNewDashboard = async () => {
    let newDashboard = {
      name: dashboardName,
      description: dashboardDesc,
      id: "",
      data_sources: [],
      controllers: [],
      visualizations: [],
      pages: [],
    };
    isDashboardSelected.set(true);
    currDashboard.update(($d) => ({
      ...$d,
      name:dashboardName,
      description:dashboardDesc,
      file_path:dashboardLoc,
    }));
    console.log(dashboardName);
    goto("/dashboards/dashboardcreator")
  };

  const createDashboardFolder = async () =>{
    try {
      const selectedFolder = await open_t({
        multiple:false,
        directory:true,
        defaultPath: await documentDir()
      });
      if(!selectedFolder){
        return;
      }
      dashboardLoc = selectedFolder as string;
    } catch (error) {
      console.log(error);
    }

  }
</script>

<div
  use:melt={$trigger}
  class="text-white bg-slate-800 hover:bg-slate-500 flex justify-start items-center px-4 py-2 my-2 text-2xl font-bold"
>
  <FolderPlus class="mr-2 text-green-600" /> Create new dashboard
</div>
{#if open}
  <div class="" use:melt={$portalled}>
    <div use:melt={$overlay} class="fixed inset-0 z-50 bg-black/50" />
    <div
      class="fixed left-1/2 top-1/2 z-50 max-h-[85vh] w-[90vw]
            max-w-[600px] -translate-x-1/2 -translate-y-1/2 bg-white
            p-6 shadow-lg"
      use:melt={$content}
    >
      <h2 use:melt={$title} class="m-0 text-lg font-medium text-black">
        Create new dashboard
      </h2>

      <fieldset class="mb-4 flex items-center gap-5">
        <Label class="w-24 text-right text-black" for="dashName"
          >Name:</Label
        >
        <Input
          class="inline-flex h-8 w-full flex-1 items-center justify-center
                  rounded-sm border border-solid px-3 leading-none text-white"
          id="dashName"
          bind:value={dashboardName}
          placeholder="Dashboard name"
        />
      </fieldset>
      <fieldset class="mb-4 flex items-center gap-5">
        <Label for="dashDesc" class="w-24 text-right text-black"
          >Description:</Label
        >
        <Textarea
          placeholder="Type description here"
          id="dashDesc"
          class="inline-flex h-8 w-full flex-1 items-center justify-center
        rounded-sm border border-solid px-3 leading-none text-white"
          bind:value={dashboardDesc}
        />
      </fieldset>
      <fieldset class="mb-4 flex items-center gap-5">
        <Label for="dashDesc" class="w-24 text-right text-black"
          >Location:</Label
        >
        <Input
          placeholder="Select folder for dashboard artifacts"
          id="dashLoc"
          class="inline-flex h-8 w-full flex-1 items-center justify-center
        rounded-sm border border-solid px-3 leading-none text-white"
        on:click={createDashboardFolder}
          bind:value={dashboardLoc}
        />
      </fieldset>
      <div class="mt-6 flex justify-end gap-4">
        <button
          use:melt={$close}
          class="inline-flex h-8 items-center justify-center rounded-sm
                    bg-red-800 px-4 font-medium leading-none text-zinc-100"
        >
          Cancel
        </button>
        <button
          on:click={createNewDashboard}
          use:melt={$close}
          class="inline-flex h-8 items-center justify-center rounded-sm
                    bg-magnum-800 px-4 font-medium leading-none text-white"
        >
          Create Dashboard
        </button>
      </div>
      <button
        use:melt={$close}
        aria-label="close"
        class="absolute right-4 top-4 inline-flex h-6 w-6 appearance-none
                items-center justify-center rounded-full p-1 text-magnum-800
                hover:bg-magnum-100 focus:shadow-magnum-400"
      >
        <X class="size-4" />
      </button>
    </div>
  </div>
{/if}
