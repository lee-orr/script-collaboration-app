import type { Message } from './Message'
import type { NetworkAdapter } from './NetworkAdapter'
import type { FileList, FileListing } from './FileList'

export default function createHostFileList(
	adapter: NetworkAdapter<Message>,
	local: FileList
): FileList {
	adapter.setListener(message => {
		switch (message.type) {
			case 'CreateFile':
				void local.createFile(message.name, message.fileType, message.key)
				break
			case 'GetCurrentList':
				adapter.sendMessage({
					type: 'CurrentListResult',
					listings: local.getCurrentList()
				})
				break
			case 'RenameFile':
				void local.renameFile(message.key, message.name)
				break
			case 'DeleteFile':
				void local.deleteFile(message.key)
				break
			default:
				break
		}
	})

	return {
		setCallback(callback): void {
			local.setCallback(listings => {
				callback(listings)
				adapter.sendMessage({ type: 'CurrentListResult', listings })
			})
		},
		async createFile(name, type): Promise<string> {
			return local.createFile(name, type)
		},
		getCurrentList(): FileListing[] {
			return local.getCurrentList()
		},
		async deleteFile(key): Promise<void> {
			void local.deleteFile(key)
		},
		async renameFile(key, name): Promise<void> {
			void local.renameFile(key, name)
		}
	}
}
