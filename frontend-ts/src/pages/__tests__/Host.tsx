import { screen } from '@testing-library/react'
import HostPage from 'pages/Host'
import renderWithProviders from 'testUtils'

describe('<Join />', () => {
	it('renders', async () => {
		renderWithProviders(<HostPage />)

		await expect(
			screen.findByText('Your Display Name:')
		).resolves.toBeInTheDocument()
		await expect(screen.findByRole('textbox')).resolves.toBeInTheDocument()
		await expect(screen.findByText('Back')).resolves.toBeInTheDocument()
		await expect(screen.findByText('Host Session')).resolves.toBeInTheDocument()
		await expect(
			screen.findByText('Choose The Hosted Folder')
		).resolves.toBeInTheDocument()
	})
})
