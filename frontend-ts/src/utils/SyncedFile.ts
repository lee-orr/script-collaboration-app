import { slateNodesToInsertDelta } from '@slate-yjs/core'
import * as Y from 'yjs'

export interface SyncedFile {
	connect: () => { id: number; content: Y.XmlText }
	disconnect: (id: number) => void
	doc: () => Y.Doc
	updateAll: (update: Uint8Array) => void
}

export function createInMemoryFile(
	name: string,
	initialContent: string
): SyncedFile {
	const mainDocument = new Y.Doc()
	const contentType = mainDocument.get('content', Y.XmlText) as Y.XmlText
	contentType.applyDelta(
		slateNodesToInsertDelta([
			{ type: 'raw', children: [{ text: initialContent }] }
		])
	)

	let syncedDocuments: Record<string, Y.Doc> = {}

	return {
		doc(): Y.Doc {
			return mainDocument
		},
		connect(): { id: number; content: Y.XmlText } {
			const document = new Y.Doc()
			const state = Y.encodeStateAsUpdate(mainDocument)
			Y.applyUpdate(document, state)
			document.on('update', (update: Uint8Array) => {
				Y.applyUpdate(mainDocument, update)
				for (const index of Object.keys(syncedDocuments)) {
					const target = syncedDocuments[index]
					Y.applyUpdate(target, update)
				}
			})
			syncedDocuments = {
				...syncedDocuments,
				[document.clientID]: document
			}
			return {
				id: document.clientID,
				content: document.get('content', Y.XmlText) as Y.XmlText
			}
		},
		updateAll(update): void {
			Y.applyUpdate(mainDocument, update)
			for (const index of Object.keys(syncedDocuments)) {
				const target = syncedDocuments[index]
				Y.applyUpdate(target, update)
			}
		},
		disconnect(id): void {
			const index = id.toString()
			if (index in syncedDocuments) {
				const document = syncedDocuments[index]
				const oldDocuments = syncedDocuments
				syncedDocuments = {}
				for (const key of Object.keys(oldDocuments)) {
					if (key !== index) {
						syncedDocuments[key] = oldDocuments[key]
					}
				}
				document.destroy()
			}
		}
	}
}
