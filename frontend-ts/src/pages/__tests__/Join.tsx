import { screen } from '@testing-library/react'
import JoinPage from 'pages/Join'
import renderWithProviders from 'testUtils'

describe('<Join />', () => {
	it('renders', async () => {
		renderWithProviders(<JoinPage />)

		await expect(
			screen.findByText('Session Code:')
		).resolves.toBeInTheDocument()
		await expect(screen.findAllByRole('textbox')).resolves.toHaveLength(2)
		await expect(screen.findByText('Back')).resolves.toBeInTheDocument()
		await expect(screen.findByText('Join Session')).resolves.toBeInTheDocument()
		await expect(screen.findByText('Join')).resolves.toBeInTheDocument()
		await expect(
			screen.findByText('Your Display Name:')
		).resolves.toBeInTheDocument()
	})
})
