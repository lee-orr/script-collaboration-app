import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import HostPage from 'pages/Host'
import renderWithProviders from 'testUtils'

describe('<Host />', () => {
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
	it('chose hosted folder is disabled until a display name is set', async () => {
		renderWithProviders(<HostPage />)
		await expect(
			screen.findByText('Choose The Hosted Folder')
		).resolves.toBeDisabled()

		const name = await screen.findByRole('textbox')
		await userEvent.type(name, 'name')

		await expect(
			screen.findByText('Choose The Hosted Folder')
		).resolves.toBeEnabled()
	})
})
