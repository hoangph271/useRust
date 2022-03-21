import { render, screen } from '@testing-library/react';
import { CanonFtb } from './CanonFtb'

test('renders CanonFtb shutter button', () => {
  render(<CanonFtb />);
  const shutterButton = screen.getByTestId('shutter-button');

  expect(shutterButton).toBeInTheDocument();
  expect(shutterButton).toHaveFocus()
});
