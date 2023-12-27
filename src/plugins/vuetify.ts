// Styles
import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";

// Vuetify
// "moduleResolution": "node"
import { createVuetify } from "vuetify";
import * as labsComponents from "vuetify/labs/components";
import * as directives from "vuetify/directives";

import * as components from "vuetify/components";

const vuetify = createVuetify({
  components: {
    ...components,
    ...labsComponents,
  },
  directives,
  theme: {
    defaultTheme: "dark",
  },
});

export default vuetify;
// https://vuetifyjs.com/en/introduction/why-vuetify/#feature-guides
