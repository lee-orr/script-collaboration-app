import type { Message } from "./Message";
import type { NetworkAdapter } from "./NetworkAdapter";
import type { FileList, FileListing } from "./FileList"; 
import { GenerateKey } from "./KeyGenerator";

export default function createNetworkedFileList(adapter: NetworkAdapter<Message>): FileList {
	let listings : FileListing[] = []
	let update: ((files: FileListing[]) => void) | undefined
    

    adapter.setListener((message) => {
        if (message.type === "CurrentListResult") {
            listings = message.listings
            if (update) update(listings)
        }
    })

    adapter.sendMessage({type: "GetCurrentList"})

	return {
		setCallback(callback): void {
			update = callback
		},
		async createFile(name, type): Promise<string> {
            const key = GenerateKey()
			const listing = { name, key, type }
			listings = [...listings, listing]
            adapter.sendMessage({
                type: "CreateFile",
                name,
                fileType: type,
                key
            })
			if (update) update(listings)
			return key
		},
		getCurrentList(): FileListing[] {
			return listings
		},
		async deleteFile(key): Promise<void> {
            adapter.sendMessage({
                type: "DeleteFile",
                key
            })
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
            adapter.sendMessage({
                type: "RenameFile",
                name,
                key
            })
			/* eslint-disable @typescript-eslint/no-unnecessary-condition */
			if (current && update) update(listings)
		}
	}
}