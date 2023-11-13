<script>
	import { processData } from "$lib/js/processdata.js";
	import { formatDate } from "$lib/js/date.js";
	import "$lib/css/base.css";
	import "$lib/css/cv.css";
	import SvgIcon from "@jamescoyle/svelte-icon";

	// Sidebar
	import profile_pic from "$lib/img/cvnQU1-W_nowhite_carre.jpg";
	import SidebarComponent from "$lib/components/sidebar-component.svelte";
	import {
		mdiAccount,
		mdiCogs,
		mdiEmailOutline,
		mdiPhone,
		mdiStar,
	} from "@mdi/js";

	// Main
	import Section from "$lib/components/section.svelte";
	import Education from "$lib/components/education.svelte";
	import { mdiSchool, mdiArrowRight, mdiChevronLeft } from "@mdi/js";

	export let data;
	const cv = data.status == 0 ? processData(data) : undefined;
	const birth_year =
		data.status == 0 ? formatDate(cv.info.birth_year) : undefined;
	let education_index = 0;

	function slideEducation() {
		const educationElements = document.querySelectorAll(".education-main");
		if (education_index >= cv.education.length - 1) {
			education_index = 0;
		} else {
			education_index++;
		}
		const transformValue = `translateX(-${education_index * 53}rem)`;
		educationElements.forEach((element) => {
			element.style.transform = transformValue;
		});
	}
</script>

{#if data.status == 0}
	<div class="container-cv">
		<!-- SIDEBAR DIV (LEFT) -->
		<div class="sidebar">
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
		<!-- MAIN DIV (RIGHT) -->
		<div class="main">
			<h1 class="name">{cv.info.full_name}</h1>
			<h2 class="name">Apprentice Engineer Automatic/Electronic</h2>
			<Section icon={mdiSchool} title="Education" />
			<div class="education">
				<button class="education_btn" on:click={slideEducation}>
					<div class="education_btn_icon">
						<SvgIcon
							size="75"
							path={education_index >= cv.education.length - 1
								? mdiChevronLeft
								: mdiArrowRight}
							type="mdi"
						/>
					</div>
				</button>
				{#each cv.education.reverse() as ed_data, index (index)}
					<Education
						data={ed_data}
						active={index == education_index ? true : false}
					/>
				{/each}
			</div>
		</div>
	</div>
{:else}
	<h1 class="h1 text-center">Oops, could not load database :/</h1>
{/if}
