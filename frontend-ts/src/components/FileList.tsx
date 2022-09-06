import type { ReactElement } from 'react'
import { useState } from 'react'
import type { FileList } from 'utils/FileList'
import { FileType } from 'utils/FileList'
import Button from './Button'

export default function FileListComponent({
	list,
	selectFile
}: {
	list: FileList
	selectFile: (key: string) => void
}): ReactElement {
	const [files, setFiles] = useState(list.getCurrentList())
	list.setCallback(setFiles)

	return (
		<div className='flex flex-col justify-start gap-4'>
			<div className='flex flex-col items-stretch gap-1'>
				<Button
					label='New Script'
					click={(): void => {
						void list.createFile('untitled script', FileType.Fountain)
					}}
				/>
				<Button
					label='New Markdown'
					click={(): void => {
						void list.createFile('untitled markdown', FileType.Markdown)
					}}
				/>
				<Button label='Upload File' click={(): void => {}} />
			</div>
			<div className='flex flex-grow flex-col items-stretch gap-1'>
				{files.map(listing => (
					<Button
						key={listing.key}
						label={listing.name}
						click={(): void => {
							selectFile(listing.key)
						}}
					/>
				))}
			</div>
		</div>
	)
}
