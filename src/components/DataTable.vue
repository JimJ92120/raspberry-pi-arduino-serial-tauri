<template>
  <v-data-table :headers="headers" :items="rows" item-value="name" />
</template>

<script lang="ts">
import { defineComponent } from "vue";

interface IRow {
  id: string;
  title: string;
  others: string;
}

interface IData {
  headers: any[];
  rows: IRow[];
}

export default defineComponent({
  name: "DataTable",

  data(): IData {
    return {
      headers: [
        { title: "ID", key: "id", align: "end" },
        { title: "Title", key: "title", align: "end" },
        { title: "Others", key: "others", align: "end" },
      ],
      rows: [
        {
          id: "1",
          title: "ABC",
          others: "",
        },
        {
          id: "2",
          title: "DEF",
          others: "Lorem ipsum",
        },
      ],
    };
  },
});
</script>
