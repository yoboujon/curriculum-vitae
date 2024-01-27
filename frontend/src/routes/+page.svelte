<script>
	import SvgIcon from "@jamescoyle/svelte-icon";
	import { processData } from "$lib/js/processdata.js";
	import { formatDate } from "$lib/js/date.js";
	import "$lib/css/base.css";
	import "$lib/css/cv.css";

	// Sidebar
	import profile_pic from "$lib/img/cvnQU1-W_nowhite_carre.jpg";
	import SidebarComponent from "$lib/components/sidebar-component.svelte";
	import {
		mdiAccount,
		mdiCogs,
		mdiEmailOutline,
		mdiPhone,
		mdiStar,
		mdiXml,
		mdiApplication,
		mdiEarth,
		mdiHeart
	} from "@mdi/js";

	// Main
	import Section from "$lib/components/section.svelte";
	import SubSection from "$lib/components/subsection.svelte";
	import Education from "$lib/components/education.svelte";
	import Experience from "$lib/components/experience.svelte";
	import Projects from "$lib/components/projects.svelte"
	import SlideShow from "$lib/components/slideshow.svelte";
	import Pill from "$lib/components/pill.svelte"
	import FlagComponent from "$lib/components/flag-component.svelte"
	import ProjectsPopup from "$lib/components/projects-popup.svelte"
	import { mdiSchool, mdiBriefcase, mdiWrench, mdiPencil } from "@mdi/js";
	import { onMount } from "svelte";

	export let data;
	const cv = data.status == 0 ? processData(data) : undefined;
	const birth_year =
		data.status == 0 ? formatDate(cv.info.birth_year) : undefined;

	// Sidebar sticky
	let sidebar;
	let footer;
	let moving_position = 0;
	$: scrollY = 0;
	$: innerHeight = 0;
	onMount(() => {
		sidebarScrollingHandler();
	});

	function sidebarScrollingHandler() {
		let isBottom = (scrollY+innerHeight  >= footer.offsetTop);
		// Testing if sidebar is outside of scrolling scope
		if(scrollY+innerHeight >= sidebar.offsetHeight && !isBottom) {
			sidebar.style.position = 'fixed';
			sidebar.style.top = `-${sidebar.offsetHeight-innerHeight}px`;
			moving_position = scrollY;
		}
		else if(isBottom) {
			sidebar.style.position = 'absolute';
			sidebar.style.top = `${moving_position-(sidebar.offsetHeight-innerHeight)}px`;
		}
		else {
			sidebar.style.position = 'absolute';
			sidebar.style.top = '';
		}
	}

	// Mobile check
	$: innerWidth = 0;
	function mobileCheck() {
		//console.log(innerWidth);
	}
</script>

<svelte:window bind:scrollY bind:innerHeight bind:innerWidth on:scroll={sidebarScrollingHandler} on:resize={mobileCheck} />

{#if data.status == 0}
	<ProjectsPopup/>
	<div class="container-cv">
		<!-- SIDEBAR DIV (LEFT) -->
		<div class="sidebar" bind:this={sidebar}>
			<div class="profile-picture-container">
				<img
					class="profile-picture"
					src={profile_pic}
					alt={cv.info.full_name}
				/>
			</div>
			<SidebarComponent icon={mdiAccount} description={birth_year} />
			<SidebarComponent
				icon={mdiEmailOutline}
				description={cv.info.email}
			/>
			<SidebarComponent
				icon={mdiPhone}
				description={cv.info.phone_number}
			/>
			<SidebarComponent
				icon={mdiStar}
				title="Interests"
				description={cv.info.interests}
			/>
			<SidebarComponent
				icon={mdiCogs}
				title="Soft-Skills"
				description={cv.info.softskills}
			/>
		</div>
		<div class="fake-sidebar" />
		<!-- MAIN DIV (RIGHT) -->
		<div class="main">
			<h1 class="name">{cv.info.full_name}</h1>
			<h2 class="name">Apprentice Engineer Automatic/Electronic</h2>
			<Section icon={mdiSchool} title="Education" />
			<SlideShow
				data={cv.education}
				type={Education}
				timeline="true"
				reverse="true"
			/>
			<Section icon={mdiBriefcase} title="Experience" />
			<SlideShow
				data={cv.experiences}
				type={Experience}
				timeline="true"
				reverse="true"
			/>
			<Section icon={mdiWrench} title="Projects" />
			<SlideShow
				data={cv.skills.project}
				type={Projects}
			/>
			<Section icon={mdiPencil} title="Skills" />
			<SubSection icon={mdiXml} title="Programming Languages"/>
			<div class="subsection">
				{#each cv.skills.programming_languages as pilldata}
					<Pill name={pilldata.lang} type_icon={pilldata.type_icon} icon={pilldata.icon} color={pilldata.color}/>
				{/each}
			</div>
			<SubSection icon={mdiApplication} title="Software"/>
			<div class="subsection">
				{#each cv.skills.softwares as pilldata}
					<Pill name={pilldata.software} type_icon={pilldata.type_icon} icon={pilldata.icon} color={pilldata.color}/>
				{/each}
			</div>
			<SubSection icon={mdiEarth} title="Languages"/>
			<div class="subsection flag-container end">
				{#each cv.skills.languages as langdata}
					<FlagComponent lang={langdata.lang} level={langdata.level} icon={langdata.icon_alpha}/>
				{/each}
			</div>
		</div>
	</div>
	<div class="footer" bind:this={footer}>
		<p>Made with <SvgIcon size="20" path={mdiHeart} type="mdi"/> using Svelte</p>
		<p>All rights reserved, Yohan Boujon • {new Date().getFullYear()}</p>
	</div>
{:else}
	<h1 class="h1 text-center">Oops, could not load database :/</h1>
{/if}
