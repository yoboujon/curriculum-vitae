<script>
	import { processData } from "$lib/js/processdata.js";
	import { formatDate } from "$lib/js/date.js";
	import "$lib/css/base.css";
	import "$lib/css/cv.css";

	// Sidebar
	import profile_pic from "$lib/img/cvnQU1-W_nowhite_carre.jpg";
	import SidebarComponent from "$lib/components/sidebar-component.svelte";
	import { mdiAccount, mdiCogs, mdiEmailOutline, mdiLocationEnter, mdiMapMarker, mdiPhone, mdiStar } from '@mdi/js';

	export let data;
	let cv = processData(data);
	let birth_year = formatDate(cv.info.birth_year);
</script>

{#if data.status == 0}
	<div class="container-cv">
		<div class="sidebar">
			<div class="profile-picture-container">
				<img class="profile-picture" src={profile_pic} alt="{cv.info.full_name}"/>
			</div>
			<SidebarComponent icon={mdiAccount} description={birth_year}/>
			<SidebarComponent icon={mdiEmailOutline} description={cv.info.email}/>
			<SidebarComponent icon={mdiPhone} description={cv.info.phone_number}/>
			<SidebarComponent icon={mdiStar} title="Interests" description={cv.info.interests}/>
			<SidebarComponent icon={mdiCogs} title="Soft-Skills" description={cv.info.softskills}/>
		</div>
		<div class="main">
			<h1 class="name">{cv.info.full_name}</h1>
			<h2 class="name">Apprentice Engineer Automatic/Electronic</h2>
		</div>
	</div>
{:else}
	<h1 class="h1 text-center">Oops, could not load database :/</h1>
{/if}
