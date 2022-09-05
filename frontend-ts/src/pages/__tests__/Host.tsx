import { fireEvent, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import HostPage from 'pages/Host'
import { act } from 'react-dom/test-utils'
import renderWithProviders from 'testUtils'
import { createInMemoryProjectList } from 'utils/ProjectList'

describe('<Host />', () => {
	it('renders', async () => {
		renderWithProviders(<HostPage projects={createInMemoryProjectList([])} />)

		await expect(
			screen.findByText('Your Display Name:')
		).resolves.toBeInTheDocument()
		await expect(screen.findAllByRole('textbox')).resolves.toHaveLength(2)
		await expect(screen.findByText('Back')).resolves.toBeInTheDocument()
		await expect(screen.findByText('Host Session')).resolves.toBeInTheDocument()
		await expect(
			screen.findByText('Start Session')
		).resolves.toBeInTheDocument()
	})
	it('chose hosted folder is disabled until a display name & project is set', async () => {
		renderWithProviders(<HostPage projects={createInMemoryProjectList([])} />)
		await expect(screen.findByText('Start Session')).resolves.toBeDisabled()

		const [name, project] = await screen.findAllByRole('textbox')
		await userEvent.type(name, 'name')
		await userEvent.type(project, 'project')

		await expect(screen.findByText('Start Session')).resolves.toBeEnabled()
	})
	it('lists available projects', async() => {
		renderWithProviders(<HostPage projects={createInMemoryProjectList([{name: 'Project', key: 'project'}])} />)
		await expect(screen.findByText('Project')).resolves.toBeInTheDocument()
	})
	it('triggers project creation, if a project didnt exist', async () => {
		let projects = createInMemoryProjectList([])
		renderWithProviders(<HostPage projects={projects} />)

		const [name, project] = await screen.findAllByRole('textbox')
		await userEvent.type(name, 'name')
		await userEvent.type(project, 'Project With A Name')

		await userEvent.click(await screen.findByText('Start Session'))
		
		expect(projects.list[0].name).toBe('Project With A Name')
		expect(projects.list[0].key).toBe('project-with-a-name')
		expect(window.location.href).toContain('/host/project-with-a-name/name')
	})
})
