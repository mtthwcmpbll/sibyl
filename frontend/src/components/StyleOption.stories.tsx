import type { Meta, StoryObj } from "@storybook/react";

import type { StyleOption as StyleOptionData, Suit } from "../platform";
import { StyleOption } from "./StyleOption";

const SUITS: Suit[] = ["cups", "wands", "swords", "pentacles"];

const sampleOption: StyleOptionData = {
  id: "r0-opt0",
  label: "Option A",
  promptUsed: "rules:line art | style:indigo and gold",
  suitIcons: SUITS.map((suit) => ({ suit, imageKey: `mock/story/${suit}.png` })),
};

const meta: Meta<typeof StyleOption> = {
  title: "Components/StyleOption",
  component: StyleOption,
  args: { option: sampleOption, selected: false, onSelect: () => {} },
};
export default meta;

type Story = StoryObj<typeof StyleOption>;

export const Default: Story = {};
export const Selected: Story = { args: { selected: true } };
