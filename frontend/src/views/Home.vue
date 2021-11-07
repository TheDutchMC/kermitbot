<template>
    <v-container>
        <v-snackbar
            v-model="snackbar">
            {{snackbarText}}
            <template v-slot:action="{ attrs }">
                <v-btn
                    color="primary"
                    text
                    v-bind="attrs"
                    @click="snackbar = false">
                    <v-icon>mdi-close</v-icon>
                </v-btn>
            </template>
        </v-snackbar>

        <v-card class="mx-auto mt-6" elevation="2">
            <v-card-title>UwU Counter </v-card-title>
            <v-data-table
                :headers="headers"
                :items="counts"
                :loading="loading"
                :hide-default-footer="true"
                sort-by.sync="count"
                sort-desc.sync="desc">
            
            </v-data-table>
        </v-card>
    </v-container>
</template>

<script lang="ts">
import Vue from 'vue'
import { SERVER } from '@/main'
import { DataTableHeader } from 'vuetify'

interface Data {
    counts:         Count[]
    loading:        boolean,
    snackbar:       boolean,
    snackbarText:   string
    headers:        DataTableHeader[]
}

interface Count {
    count:  number,
    name:   string
} 

export default Vue.extend({
    data(): Data {
        return {
            counts: [],
            loading: true,
            snackbar: false,
            snackbarText: null as any,
            headers: [
                {
                    text: 'User',
                    value: 'name'
                },
                {
                    text: 'UwU Count',
                    value: 'count'
                }
            ]
        }
    },
    mounted() {
        this.fetchData()
    },
    methods: {
        fetchData() {
            fetch(`${SERVER}/uwu-counter`)
            .then(r => {
                if(!r.ok) {
                    throw r
                }

                switch(r.status) {
                    case 200:
                        r.json().then(j => {
                            interface Response {
                                users: Count[]
                            }

                            this.counts = (<Response> j).users
                            this.loading = false
                        })  
                        break
                    default:
                        this.snackbarText = 'Something went wrong, please try again later.'
                        this.snackbar = true
                        this.loading = false
                }
            })
        }
    }
})

</script>