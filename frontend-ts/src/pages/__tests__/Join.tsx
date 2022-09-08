import { screen } from '@testing-library/react'
import JoinPage from 'pages/Join'
import renderWithProviders from 'testUtils'
import userEvent from '@testing-library/user-event'

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
	it('navigates to correct session page', async () => {
		renderWithProviders(<JoinPage />)
		const [code, name] = await screen.findAllByRole('textbox')

		await userEvent.type(code, 'code')
		await userEvent.type(name, 'name')

		await userEvent.click(await screen.findByText('Join'))

		expect(window.location.href).toContain('/session/code/name')
	})
	it('join is disabled until a display name & code are set', async () => {
		renderWithProviders(<JoinPage />)
		await expect(screen.findByText('Join')).resolves.toBeDisabled()

		const [code, name] = await screen.findAllByRole('textbox')

		await userEvent.type(code, 'code')
		await expect(screen.findByText('Join')).resolves.toBeDisabled()
		await userEvent.type(name, 'name')

		await expect(screen.findByText('Join')).resolves.toBeEnabled()
	})
})
