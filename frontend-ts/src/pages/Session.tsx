import type { ReactElement } from 'react'
import { useMemo, useState } from 'react'
import { useParams } from 'react-router-dom'
import Split from 'react-split'
import FileListComponent from 'components/FileList'
import type { FileList, FileListing } from 'utils/FileList'
import { createInMemoryFileList } from 'utils/FileList'
import Editor from './Editor'
import type { SyncedFile } from 'utils/SyncedFile'
import { createInMemoryFile } from 'utils/SyncedFile'
import LoadingOrError from 'components/LoadingOrError'
import createIdbFileList from 'utils/IdbFileList'
import getIdbFile from 'utils/IdbSyncedFile'

export default function SessionPage({
	isHost
}: {
	isHost: boolean
}): ReactElement {
	const [files, setFiles] = useState<FileList>()
	const { name, project, code } = useParams<{
		name: string | undefined
		project: string | undefined
		code: string | undefined
	}>()
	const [openFiles, setOpenFiles] = useState<
		Record<string, boolean | { listing: FileListing; file: SyncedFile }>
	>({})

	void useMemo(async () => {
		if (project) {
			const fileList = await createIdbFileList(project)
			setFiles(fileList)
		} else {
			const fileList = createInMemoryFileList([])
			setFiles(fileList)
		}
	}, [project])

	if (!files) {
		return <LoadingOrError />
	}

	const openFileList = Object.keys(openFiles)
		.map(key => openFiles[key])
		.filter(v => v !== false) as { listing: FileListing; file: SyncedFile }[]

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
							if (file.key in openFiles) return
							{
								const open = { ...openFiles, [file.key]: false }
								setOpenFiles(open)
							}
							if (project) {
								void getIdbFile(file.key, project).then(value => {
									const open = {
										...openFiles,
										[file.key]: {
											listing: file,
											file: value
										}
									}
									setOpenFiles(open)
								})
							} else {
								const value = createInMemoryFile('file', '')
								const open = {
									...openFiles,
									[file.key]: {
										listing: file,
										file: value
									}
								}
								setOpenFiles(open)
							}
						}}
					/>
				</div>
				<Split
					className='split flex flex-grow flex-row'
					key={openFileList.map(v => v.listing.key).join(':')}
				>
					{openFileList.length > 0 ? (
						openFileList.map(({ listing, file }) => (
							<Editor
								key={listing.key}
								listing={listing}
								file={file}
								renameFile={(updatedName): void => {
									const open = {
										...openFiles,
										[listing.key]: {
											file,
											listing: {
												...listing,
												name: updatedName
											}
										}
									}
									setOpenFiles(open)
									void files.renameFile(listing.key, updatedName)
								}}
								closeFile={(): void => {
									const open: Record<
										string,
										boolean | { listing: FileListing; file: SyncedFile }
									> = {}
									for (const key of Object.keys(openFiles)) {
										if (key !== listing.key) {
											open[key] = openFiles[key]
										}
									}
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
