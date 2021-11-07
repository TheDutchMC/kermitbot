import Vue from 'vue'
import App from './App.vue'
import router from './router'
import vuetify from './plugins/vuetify'

Vue.config.productionTip = false

export const SERVER = process.env.NODE_ENV === 'production' ? "/api/v1" : "http://localhost:8080/api/v1"


new Vue({
	router,
	vuetify,
	render: h => h(App)
}).$mount('#app')
