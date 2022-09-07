import type { FileList, FileListing } from './FileList'
import getDatabase from './Idb'
import { GenerateKey } from './KeyGenerator'

export default async function createIdbFileList(
	project: string
): Promise<FileList> {
	const database = await getDatabase(project)

	let listings = await database.getAll('fileInfo')
	let update: ((files: FileListing[]) => void) | undefined

	return {
		setCallback(callback): void {
			update = callback
		},
		async createFile(name, type): Promise<string> {
			const key = GenerateKey()
			const listing = { name, key, type }
			const tx = database.transaction(['fileInfo', 'fileContent'], 'readwrite')
			void tx.objectStore('fileInfo').put(listing, key)
			void tx.objectStore('fileContent').put(new Uint8Array(), key)
			await tx.done
			listings = [...listings, listing]
			if (update) update(listings)
			return key
		},
		getCurrentList(): FileListing[] {
			return listings
		},
		async deleteFile(key): Promise<void> {
			const tx = database.transaction(['fileInfo', 'fileContent'], 'readwrite')
			void tx.objectStore('fileInfo').delete(key)
			void tx.objectStore('fileContent').delete(key)
			await tx.done
			listings = listings.filter(p => p.key !== key)
			if (update) update(listings)
		},
		async renameFile(key, name): Promise<void> {
			let current: FileListing | false = false
			listings = listings.map(listing => {
				if (listing.key === key) {
					current = { ...listing, name }
					return current
				}
				return listing
			})
			/* eslint-disable @typescript-eslint/no-unnecessary-condition */
			if (current) {
				await database.put('fileInfo', current, key)
				if (update) update(listings)
			}
		}
	}
}
