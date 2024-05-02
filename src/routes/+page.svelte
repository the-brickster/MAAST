<script>

	import { invoke } from "@tauri-apps/api/tauri";
	import { listen } from "@tauri-apps/api/event";
	import { event } from "@tauri-apps/api";

	import { Button } from "$lib/components/ui/button";
	import * as Card from "$lib/components/ui/card";
	import {Label} from "$lib/components/ui/label";
	import {Separator} from "$lib/components/ui/separator";
	import {ScrollArea} from "$lib/components/ui/scroll-area"
    import * as Accordion from "$lib/components/ui/accordion";

	import {versionsData} from "../constants/versions";
	import {dashboardSelected,dashboardStore} from '$lib/stores';
    import { keyed } from "$lib/keyed";


	export let data;
	console.log(data);

	let counter = 0;
	let idVal = "";
	let increment = (addString) => {
		counter++;
		idVal = addString+counter;
		return idVal;
	};
	const getCount = counter;

	let sendVal = "THIS IS NOT A DRILL!";

	let return_val = sendOutput();
	async function sendOutput() {
		console.log("JS: js2rs: " + sendVal);
		const res = invoke("js2rs", { payload: sendVal });
		return res;
	}

	let event_val = getEventPayload();
	let event_output = "";
	async function getEventPayload() {
		return await listen("rs2js", (event) => {
			console.log(event);
			event_output = event.payload;
			console.log(event_output);
		});
	}


  export const dBoardName = keyed(dashboardStore,"name");
</script>

<svelte:head>
	<title>Home</title>
	<meta name="MAAST" content="MA Analysis and Statistics Tool" />
</svelte:head>

<section>
	<div
		class="flex flex-wrap min-h-[calc(100vh_-_theme(spacing.16))] flex-1 flex-col gap-4 bg-muted/40 p-4 md:gap-8 md:p-10 select-none"
	>
		<div class="mx-auto grid w-full max-w-6xl gap-2">
			<h1 class="text-4xl font-semibold">Welcome to MAAST</h1>
		</div>
		<p>{JSON.stringify($dashboardStore)}</p>
		<p>{$dashboardSelected}</p>
		<div
			class="mx-auto grid w-full max-w-6xl items-start gap-6 md:grid-cols-[180px_1fr] lg:grid-cols-[350px_1fr]"
		>
			<div class="grid gap-2 text-sm text-muted-foreground">
				<h3 class="text-2xl font-semibold">Recent Dashboards</h3>
				{#each data.projects as dashboard}
				
				<div class="flex gap-2 grid-cols-1 overflow-hidden whitespace-nowrap">
					<Label class="leading-snug" for="{increment("dashboards")}">{dashboard.dashboardName}</Label>
					<a	id="{idVal}"
						title={dashboard.urlPath}
						class="overflow-hidden whitespace-nowrap text-ellipsis font-medium text-blue-600 dark:text-blue-500 hover:underline"
						href="##">{dashboard.urlPath}</a
					>
				</div>
					
				
				{/each}
				
			</div>
			<div class="grid gap-6">
				<Card.Root
					data-x-chunk-name="dashboard-04-chunk-1"
					data-x-chunk-description="A form to update the store name."
				>
					<Card.Header>
						<Card.Title>Changelog</Card.Title>
					</Card.Header>
					<Card.Content>
						<ScrollArea class="h-80 w-full rounded-md border">
							<div class="p-4">
								<Accordion.Root class="w-full">
									{#each versionsData as v}
										<Accordion.Item value="item-{v.versionTag}">
											<Accordion.Trigger>{v.versionTag}</Accordion.Trigger>
											<Accordion.Content>
												{#each v.changes as c}
													<p>- {c}</p>
												{/each}
											</Accordion.Content>
										</Accordion.Item>
										
									{/each}
								</Accordion.Root>

							</div>
						  </ScrollArea>
					</Card.Content>

				</Card.Root>
			</div>
		</div>
	</div>
</section>
