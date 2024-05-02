import { writable } from "svelte/store";
import type { Dashboard,Controller,Page,DataSource } from '$lib/types';


export const dashboardSelected = writable<boolean>(false);
export const dashboardStore = writable<Dashboard>({
        name: '',
         description: '',
         id: '',
         file_path:'',
         data_sources: [],
         controllers: [],
         visualizations: [],
         pages: []
});

