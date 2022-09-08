import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import MenuPage from 'pages/Menu'
import renderWithProviders, {
	MOBILE_RESOLUTION_HEIGHT,
	MOBILE_RESOLUTION_WIDTH
} from 'testUtils'

describe('<MenuPage />', () => {
	it('renders', async () => {
		renderWithProviders(<MenuPage />)

		await expect(screen.findByText('Host Session')).resolves.toBeInTheDocument()
		await expect(screen.findByText('Join Session')).resolves.toBeInTheDocument()
	})
	it('renders with mobile resolution', async () => {
		window.resizeTo(MOBILE_RESOLUTION_WIDTH, MOBILE_RESOLUTION_HEIGHT)
		renderWithProviders(<MenuPage />)

		await expect(screen.findByText('Host Session')).resolves.toBeInTheDocument()
		await expect(screen.findByText('Join Session')).resolves.toBeInTheDocument()
	})
	it('navigates to join page', async () => {
		renderWithProviders(<MenuPage />)
		await expect(screen.findByText('Join Session')).resolves.toBeInTheDocument()
		const button = await screen.findByText('Join Session')
		await userEvent.click(button)
		expect(window.location.href).toContain('/join')
	})
	it('navigates to host page', async () => {
		renderWithProviders(<MenuPage />)
		await expect(screen.findByText('Host Session')).resolves.toBeInTheDocument()
		const button = await screen.findByText('Host Session')
		await userEvent.click(button)
		expect(window.location.href).toContain('/host')
	})
})
