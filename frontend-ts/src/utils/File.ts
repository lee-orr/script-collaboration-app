import { slateNodesToInsertDelta } from '@slate-yjs/core';
import * as Y from 'yjs';

export interface File {
    connect(): { id: number, name: Y.Text, content: Y.XmlText },
    disconnect(id: number): void,
}

export function createInMemoryFile(name: string, initialContent: string): File {
    const mainDoc = new Y.Doc();
    const nameType = mainDoc.getText('name');
    const contentType = mainDoc.get('content', Y.XmlText) as Y.XmlText;

    nameType.insert(0, name)
    contentType.applyDelta(slateNodesToInsertDelta([{type: 'raw', children: [{text: initialContent}]}]))

    const childDocs : Record<number, Y.Doc> = {}

    return {
        connect() {
            const doc = new Y.Doc();
            const state = Y.encodeStateAsUpdate(mainDoc)
            Y.applyUpdate(doc, state)
            doc.on('update', (update) => {
                Y.applyUpdate(mainDoc, update)
                for (const i in childDocs) {
                    const target = childDocs[i];
                    Y.applyUpdate(target, update)
                }
            })
            childDocs[doc.clientID] = doc
            return {
                id: doc.clientID,
                name: doc.getText('name'),
                content: doc.get('content', Y.XmlText) as Y.XmlText
            }
        },
        disconnect(id) {
            if (id in childDocs) {
                const doc = childDocs[id]
                delete childDocs[id]
                doc.destroy()
            }
        }
    }
}