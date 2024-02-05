import React from 'react';
import { render, screen } from '@testing-library/react';
import {{componentName}} from './{{componentName}}'; // Import your component

describe('{{componentName}}', () => {
  it('should render the {{componentName}}', () => {
    render(<{{componentName}} />); // Render the component with some props
    const {{componentName}}Element = screen.getByText(/click me/i); // Query for the {{componentName}} element
    expect({{componentName}}Element).toBeInTheDocument(); // Assert that the {{componentName}} is in the document
  });
});