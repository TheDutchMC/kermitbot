<template>
	<v-app>
		<v-app-bar
			app
			color="primary"
			dark>
			<div class="d-flex align-center">
				<v-img
					alt="KermitCraft logo"
					class="shrink mr-2"
					contain
					src="@/assets/logo.png"
					transition="scale-transition"
					width="40"/>

				<div class="text-h4"> KermitCraft </div>
			</div>
		</v-app-bar>

		<v-main>
			<router-view/>
		</v-main>
	</v-app>
</template>
