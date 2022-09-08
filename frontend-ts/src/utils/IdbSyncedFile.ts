import getDatabase from './Idb'
import type { SyncedFile } from './SyncedFile'
import * as Y from 'yjs'
import { slateNodesToInsertDelta } from '@slate-yjs/core'

const files: Record<string, Promise<SyncedFile>> = {}

const SAVE_DELAY = 2000

async function internalGetIdbFile(
	key: string,
	project: string
): Promise<SyncedFile> {
	const database = await getDatabase(project)

	const initialContent = await database.get('fileContent', key)

	const mainDocument = new Y.Doc()

	if (initialContent && initialContent.byteLength > 0) {
		Y.applyUpdate(mainDocument, initialContent)
	} else {
		const contentType = mainDocument.get('content', Y.XmlText) as Y.XmlText
		contentType.applyDelta(
			slateNodesToInsertDelta([{ type: 'raw', children: [{ text: 'test' }] }])
		)
	}

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
					const snapshot = Y.encodeStateAsUpdate(mainDocument)
					void database.put('fileContent', snapshot, key)
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
				const snapshot = Y.encodeStateAsUpdate(mainDocument)
				void database.put('fileContent', snapshot, key)
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

export default async function getIdbFile(
	key: string,
	project: string
): Promise<SyncedFile> {
	if (key in files) return files[key]
	files[key] = internalGetIdbFile(key, project)
	return files[key]
}
