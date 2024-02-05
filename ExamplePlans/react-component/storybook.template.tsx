import type { Meta, StoryObj } from '@storybook/react';

import {{componentName}} from './{{componentName}}';

const meta: Meta<typeof {{componentName}}> = {
  component: {{componentName}},
};

export default meta;
type Story = StoryObj<typeof {{componentName}}>;

/*
 *👇 Render functions are a framework specific feature to allow you control on how the component renders.
 * See https://storybook.js.org/docs/api/csf
 * to learn how to use render functions.
 */
export const Primary: Story = {
  render: () => <{{componentName}} />,
};