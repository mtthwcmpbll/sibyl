import type { Meta, StoryObj } from "@storybook/react";

import type { CardStyleGuide } from "../platform";
import { StyleGuidePreview } from "./StyleGuidePreview";

const guide: CardStyleGuide = {
  id: "sg-story",
  version: 1,
  derivedFromStyleOptionId: "r0-opt0",
  promptUsed: "style guide derived from r0-opt0",
  borderChromeKey: "mock/story/border-chrome.png",
  cardBackKey: "mock/story/card-back.png",
  cardFrontLayout: {
    aspect: 0.66,
    suitIcon: { x: 0.05, y: 0.05, w: 0.18, h: 0.18 },
    cardImagery: { x: 0.1, y: 0.18, w: 0.8, h: 0.62 },
    title: { x: 0, y: 0.86, w: 1, h: 0.1 },
  },
  shaderAreas: [{ id: "uv-frame", rect: { x: 0, y: 0, w: 1, h: 1 }, kind: "foil" }],
  flourishes: [
    { assetKey: "mock/story/flourish-1.png", rect: { x: 0, y: 0, w: 0.15, h: 0.15 }, rotation: 0 },
  ],
};

const meta: Meta<typeof StyleGuidePreview> = {
  title: "Components/StyleGuidePreview",
  component: StyleGuidePreview,
  args: { guide },
};
export default meta;

type Story = StoryObj<typeof StyleGuidePreview>;
export const Default: Story = {};
