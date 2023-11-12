<script>
	import { processData } from '$lib/processdata.js';
	export let data;
	let cv = processData(data);
</script>

<div class="my-4">
	{#if data.status == 0}
		<h1 class="h1 text-center">Le super CV de {cv.info.full_name}</h1>
	{:else}
		<h1 class="h1 text-center">Oops, could not load database :/</h1>
	{/if}
</div>
