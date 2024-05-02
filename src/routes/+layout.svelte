<script lang="ts">
	import "../app.pcss";
	import {Radius,Sun,Moon} from 'lucide-svelte';
	import {ModeWatcher,toggleMode} from "mode-watcher";
	import {Label} from "$lib/components/ui/label";
	import {Button} from "$lib/components/ui/button";
</script>
<ModeWatcher/>
<div class="flex min-h-screen w-full flex-col">
	
		<header class="top-0 flex h-10 items-center gap-4 border-b bg-background px-4 md:px-6">
			<nav class="w-full flex-grow gap-6 font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-sm lg:w-auto">
				<a href="/" class="flex items-center gap-2">
					<Radius id="logo" size={28} color="#689ff8" strokeWidth={1.75} />
					<Label class="text-lg font-bold" for="logo">MAAST</Label>
				</a>
				<a href="/dashboards" class="text-foreground transition-colors hover:text-foreground">
					Dashboards
				</a>
				<a href="/about" class="text-foreground transition-colors hover:text-foreground">
					Data Sources
				</a>
				<a href="/dashboards/dashboardcreator" class="text-foreground transition-colors hover:text-foreground">
					Common Views
				</a>
			</nav>
			<Button on:click={toggleMode} variant="ghost" size="icon" class="">
				<Sun
				class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
			  />
			  <Moon
			  class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
			/>
			</Button>
		</header> 
		<main>
			<slot></slot>
		</main>
</div>
