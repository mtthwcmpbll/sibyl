import type { Meta, StoryObj } from "@storybook/react";

import { CardFlip } from "./CardFlip";

const meta: Meta<typeof CardFlip> = {
  title: "Components/CardFlip",
  component: CardFlip,
  args: {
    frontKey: "mock/story/front.png",
    backKey: "mock/story/back.png",
    title: "queen-of-cups",
  },
};
export default meta;

type Story = StoryObj<typeof CardFlip>;

/** The card as first presented (FR-010 / SC-008): face down. */
export const FaceDown: Story = { args: { faceUp: false } };
/** After the flip: the revealed court card. */
export const Revealed: Story = { args: { faceUp: true } };
