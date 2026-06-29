import type { Preview } from "@storybook/react";
import "../src/styles.css";

const preview: Preview = {
  parameters: {
    backgrounds: {
      default: "deck",
      values: [{ name: "deck", value: "#0f0d1a" }],
    },
  },
};

export default preview;
