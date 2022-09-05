import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import App from 'App'
import renderWithProviders from 'testUtils'

describe('<App />', () => {
	it('renders main page', async () => {
		window.history.pushState({}, 'Home', '/')
		renderWithProviders(<App />, false)

		await expect(screen.findByText('Script Editor')).resolves.toBeInTheDocument()
		await expect(screen.findByText('Host Session')).resolves.toBeInTheDocument()
		await expect(screen.findByText('Join Session')).resolves.toBeInTheDocument()
	})
})
