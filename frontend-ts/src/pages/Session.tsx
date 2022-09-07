import type { ReactElement} from 'react';
import { useState } from 'react'
import { useParams } from 'react-router-dom'
import Split from 'react-split'
import FileListComponent from 'components/FileList'
import type { FileList } from 'utils/FileList'
import Editor from './Editor'

export default function SessionPage({
	isHost,
	files
}: {
	isHost: boolean
	files: FileList
}): ReactElement {
	const { name, project, code } = useParams<{
		name: string | undefined
		project: string | undefined
		code: string | undefined
	}>()
	const [openFiles, setOpenFiles] = useState<string[]>([])
	return (
		<div className='flex h-screen flex-col items-stretch justify-start'>
			<div className='flex flex-row justify-center bg-slate-900 p-2'>
				{isHost ? 'Hosting' : 'Joining'}, {name} @{' '}
				{project ?? code ?? 'No code or project'}
			</div>
			<div className='flex flex-grow flex-row'>
				<div className='flex w-56 flex-col justify-start border-r-2 border-r-slate-900 bg-slate-800 p-2'>
					<FileListComponent
						list={files}
						selectFile={(file): void => {
							if (openFiles.includes(file)) return
							const open = [...openFiles, file]
							setOpenFiles(open)
						}}
					/>
				</div>
				<Split className='split flex flex-grow flex-row' key={openFiles.length}>
					{openFiles.length > 0 ? (
						openFiles.map(f => (
								<Editor
									key={f}
									file={f}
									closeFile={(): void => {
										const open = openFiles.filter(key => key !== f)
										setOpenFiles(open)
									}}
								/>
							))
					) : (
						<div>No Open Files</div>
					)}
				</Split>
			</div>
		</div>
	)
}
