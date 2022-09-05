import { screen } from '@testing-library/react'
import MenuPage from 'pages/Menu'
import { act } from 'react-dom/test-utils'
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
		await act(async () => {
			const button = await screen.findByText('Join Session')
			button.click()
		})
		expect(window.location.href).toContain('/join')
	})
	it('navigates to host page', async () => {
		renderWithProviders(<MenuPage />)
		await expect(screen.findByText('Host Session')).resolves.toBeInTheDocument()
		await act(async () => {
			const button = await screen.findByText('Host Session')
			button.click()
		})
		expect(window.location.href).toContain('/host')
	})
})
