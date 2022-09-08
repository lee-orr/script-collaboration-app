import { GenerateKey } from './KeyGenerator'

export enum FileType {
	Fountain = 'fountain',
	Markdown = 'md'
}

export interface FileListing {
	name: string
	key: string
	type: FileType
}

export interface FileList {
	setCallback: (callback: (files: FileListing[]) => void) => void
	getCurrentList: () => FileListing[]
	createFile: (name: string, type: FileType, key?: string) => Promise<string>
	deleteFile: (key: string) => Promise<void>
	renameFile: (key: string, name: string) => Promise<void>
}

export function createInMemoryFileList(
	list: FileListing[]
): FileList & { update?: (files: FileListing[]) => void; list: FileListing[] } {
	return {
		list,
		getCurrentList(): FileListing[] {
			return list
		},
		async createFile(name: string, type: FileType): Promise<string> {
			const key = GenerateKey()
			this.list = [...this.list.filter(p => p.key !== key), { name, key, type }]
			if (this.update) {
				this.update(this.list)
			}
			return key
		},

		async deleteFile(key): Promise<void> {
			this.list = this.list.filter(p => p.key !== key)
			if (this.update) {
				this.update(this.list)
			}
		},
		setCallback(callback): void {
			this.update = callback
		},
		async renameFile(key, name): Promise<void> {
			this.list = this.list.map(listing => {
				if (listing.key === key) {
					return { ...listing, name }
				}
				return listing
			})
			if (this.update) this.update(this.list)
		}
	}
}
