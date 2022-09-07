import type { DBSchema, IDBPDatabase } from 'idb'
import { openDB } from 'idb'
import type { FileListing } from './FileList'

interface Schema extends DBSchema {
	fileInfo: {
		key: string
		value: FileListing
	}
	fileContent: {
		key: string
		value: Uint8Array
	}
}

const databases: Record<string, Promise<IDBPDatabase<Schema>>> = {}

const CURRENT_VERSION = 2

export default async function getDatabase(
	project: string
): Promise<IDBPDatabase<Schema>> {
	if (project in databases) return databases[project]
	databases[project] = openDB<Schema>(project, CURRENT_VERSION, {
		upgrade(upgradeable) {
			upgradeable.createObjectStore('fileInfo')
			upgradeable.createObjectStore('fileContent')
		}
	})
	return databases[project]
}
