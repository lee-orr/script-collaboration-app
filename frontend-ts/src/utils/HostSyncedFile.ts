import type { Message } from './Message'
import type { NetworkAdapter } from './NetworkAdapter'
import type { SyncedFile } from './SyncedFile'
import * as Y from 'yjs'
import { fromUint8Array, toUint8Array } from 'js-base64'

let adapter: NetworkAdapter<Message> | undefined

const SAVE_DELAY = 200

const files: Record<string, Promise<SyncedFile>> = {}
const documents: Record<string, Y.Doc> = {}
let currentProject = ''
let localGetter:
	| ((key: string, project: string) => Promise<SyncedFile>)
	| undefined

function internalCreateHostSyncFile(
	key: string,
	local: SyncedFile
): SyncedFile {
	const document = local.doc()
	documents[key] = document

	let timeout: number | false = false

	document.on('update', () => {
		if (timeout) {
			clearTimeout(timeout)
		}
		timeout = setTimeout(() => {
			const snapshot = fromUint8Array(Y.encodeStateAsUpdate(document))
			adapter?.sendMessage({
				type: 'FileContentUpdated',
				key,
				update: snapshot
			})
		}, SAVE_DELAY) as unknown as number
	})
	return local
}

export async function createHostSyncedFile(key: string): Promise<SyncedFile> {
	if (key in files) return files[key]
	if (!localGetter) throw new Error('No getter set')
	files[key] = localGetter(key, currentProject).then(file =>
		internalCreateHostSyncFile(key, file)
	)
	return files[key]
}

export function setHostAdapter(
	updatedAdapter: NetworkAdapter<Message>,
	project: string,
	getter: (key: string, project: string) => Promise<SyncedFile>
): void {
	adapter = updatedAdapter
	currentProject = project
	localGetter = getter
	adapter.setListener(message => {
		switch (message.type) {
			case 'FileContentUpdated':
				if (localGetter !== undefined) {
					void createHostSyncedFile(message.key).then(() => {
						const document = files[message.key]
						void document.then(file =>
							file.updateAll(toUint8Array(message.update))
						)
					})
				}
				break
			case 'RequestFileState':
				if (localGetter !== undefined) {
					void createHostSyncedFile(message.key).then(() => {
						const document = documents[message.key]
						adapter?.sendMessage({
							type: 'FullFileState',
							key: message.key,
							update: fromUint8Array(Y.encodeStateAsUpdate(document))
						})
					})
				}
				break
			default:
				break
		}
	})
}
