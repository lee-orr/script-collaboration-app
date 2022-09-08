import type { SyncedFile } from './SyncedFile'
import * as Y from 'yjs'
import type { Message } from './Message'
import type { NetworkAdapter } from './NetworkAdapter'
import { fromUint8Array, toUint8Array } from 'js-base64'

const files: Record<string, Promise<SyncedFile>> = {}

const SAVE_DELAY = 200

let adapter: NetworkAdapter<Message> | undefined

export function setClientAdapter(
	updatedAdapter: NetworkAdapter<Message>
): void {
	adapter = updatedAdapter
	adapter.setListener(message => {
		switch (message.type) {
			case 'FullFileState':
				{
					const document = files[message.key]
					/* eslint-disable @typescript-eslint/no-unnecessary-condition */
					if (document !== undefined) {
						void document.then(file =>
							file.updateAll(toUint8Array(message.update))
						)
					}
				}
				break
			case 'FileContentUpdated':
				{
					const document = files[message.key]
					/* eslint-disable @typescript-eslint/no-unnecessary-condition */
					if (document !== undefined) {
						void document.then(file =>
							file.updateAll(toUint8Array(message.update))
						)
					}
				}
				break
			default:
				break
		}
	})
}

async function internalGetNetworkSyncedFile(key: string): Promise<SyncedFile> {
	adapter?.sendMessage({
		type: 'RequestFileState',
		key
	})
	const mainDocument = new Y.Doc()

	let syncedDocuments: Record<string, Y.Doc> = {}

	let timeout: number | false = false

	return {
		doc(): Y.Doc {
			return mainDocument
		},
		connect(): { id: number; content: Y.XmlText } {
			const document = new Y.Doc()
			const state = Y.encodeStateAsUpdate(mainDocument)
			Y.applyUpdate(document, state)
			document.on('update', (update: Uint8Array) => {
				if (timeout) {
					clearTimeout(timeout)
				}
				timeout = setTimeout(() => {
					const snapshot = fromUint8Array(Y.encodeStateAsUpdate(mainDocument))
					adapter?.sendMessage({
						type: 'FileContentUpdated',
						key,
						update: snapshot
					})
				}, SAVE_DELAY) as unknown as number
				Y.applyUpdate(mainDocument, update)
				for (const index of Object.keys(syncedDocuments)) {
					const target = syncedDocuments[index]
					Y.applyUpdate(target, update)
				}
			})
			syncedDocuments[document.clientID] = document
			return {
				id: document.clientID,
				content: document.get('content', Y.XmlText) as Y.XmlText
			}
		},
		updateAll(update): void {
			if (timeout) {
				clearTimeout(timeout)
			}
			timeout = setTimeout(() => {
				const snapshot = fromUint8Array(Y.encodeStateAsUpdate(mainDocument))
				adapter?.sendMessage({
					type: 'FileContentUpdated',
					key,
					update: snapshot
				})
			}, SAVE_DELAY) as unknown as number
			Y.applyUpdate(mainDocument, update)
			for (const index of Object.keys(syncedDocuments)) {
				const target = syncedDocuments[index]
				Y.applyUpdate(target, update)
			}
		},
		disconnect(id): void {
			const index = id.toString()
			if (index in syncedDocuments) {
				const document = syncedDocuments[id]
				const oldDocuments = syncedDocuments
				syncedDocuments = {}
				for (const oldKey of Object.keys(oldDocuments)) {
					if (oldKey !== index) {
						syncedDocuments[oldKey] = oldDocuments[oldKey]
					}
				}
				document.destroy()
			}
		}
	}
}

export async function getNetworkSyncedFile(key: string): Promise<SyncedFile> {
	if (key in files) return files[key]
	files[key] = internalGetNetworkSyncedFile(key)
	return files[key]
}
