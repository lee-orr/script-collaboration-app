import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import FileList from 'components/FileList'
import { createInMemoryFileList, FileType } from 'utils/FileList'

describe('<FileList />', () => {
	it('renders', () => {
		render(
			<FileList
				list={createInMemoryFileList([
					{ name: 'Test', key: 'test', type: FileType.Fountain }
				])}
			/>
		)

		expect(screen.getByText('New Script')).toBeInTheDocument()
		expect(screen.getByText('New Markdown')).toBeInTheDocument()
		expect(screen.getByText('Upload File')).toBeInTheDocument()
		expect(screen.getByText('Test')).toBeInTheDocument()
	})
	it('creates a new script file', async () => {
		render(<FileList list={createInMemoryFileList([])} />)

		await userEvent.click(screen.getByText('New Script'))

		expect(screen.getByText('untitled script')).toBeInTheDocument()
	})
	it('creates a new markdown file', async () => {
		render(<FileList list={createInMemoryFileList([])} />)

		await userEvent.click(screen.getByText('New Markdown'))

		expect(screen.getByText('untitled markdown')).toBeInTheDocument()
	})
})
