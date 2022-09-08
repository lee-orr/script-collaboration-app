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
import { createPeerNetworkAdapter } from 'utils/NetworkAdapter'
import { Message } from 'utils/Message'
import createHostFileList from 'utils/HostFileList'
import createNetworkedFileList from 'utils/NetworkedFileList'
import { createHostSyncedFile, setHostAdapter } from 'utils/HostSyncedFile'
import {
	getNetworkSyncedFile,
	setClientAdapter
} from 'utils/NetworkedSyncedFile'

const adapter = createPeerNetworkAdapter<Message>()

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
	const [connected, setConnected] = useState<boolean | Error>(false)

	const linkCode = useMemo(() => {
		if (isHost) {
			setConnected(true)
			setHostAdapter(adapter, project ?? "", getIdbFile)
			return adapter.startHost()
		} else if (code) {
			adapter.connectionEventListener((connection, event, error) => {
				if (event === 'error') {
					setConnected(error ?? new Error('Unknown Error'))
				} else if (event === 'open') {
					setConnected(true)
				} else {
					setConnected(false)
				}
			})
			setClientAdapter(adapter)
			return adapter.startClient(code)
		}
	}, [isHost, code, project])

	void useMemo(async () => {
		if (!connected) return
		if (project) {
			const localFileList = await createIdbFileList(project)
			const fileList = createHostFileList(adapter, localFileList)
			setFiles(fileList)
		} else {
			const fileList = createNetworkedFileList(adapter)
			setFiles(fileList)
		}
	}, [project, connected])

	if (connected !== true) {
		return (
			<LoadingOrError
				error={connected instanceof Error ? connected : undefined}
			/>
		)
	}

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
				&nbsp;
				{isHost ? (
					<a
						className='text-blue-200'
						target='_blank'
						href={`${
							window.location.toString().split('/host')[0]
						}/join/${linkCode}`}
					>
						{linkCode}
					</a>
				) : (
					''
				)}
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
								void createHostSyncedFile(file.key).then(
									value => {
										const open = {
											...openFiles,
											[file.key]: {
												listing: file,
												file: value
											}
										}
										setOpenFiles(open)
									}
								)
							} else {
								void getNetworkSyncedFile(file.key).then(value => {
									const open = {
										...openFiles,
										[file.key]: {
											listing: file,
											file: value
										}
									}
									setOpenFiles(open)
								})
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
