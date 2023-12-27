<template>
  <v-app>
    <v-main>
      <h1>Tauri + VueJS</h1>
      <v-btn @click="() => toggle('on')"> On </v-btn>
      <v-btn @click="() => toggle('off')"> Off </v-btn>
      <p>{{ test }}</p>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface IData {
  test: string;
}

export default defineComponent({
  name: "App",

  data(): IData {
    return {
      test: "",
    };
  },

  async mounted() {
    // setInterval(async () => {
    //   this.$data.test = await invoke("read_serial");
    // }, 5000);
  },

  methods: {
    async toggle(action: "on" | "off") {
      await invoke("write_serial", { action: "on" === action ? "1" : "0" });
    },
  },
});
</script>
