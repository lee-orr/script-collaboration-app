import type { FileListing, FileType } from './FileList'

export interface GetCurrentList {
	type: 'GetCurrentList'
}

export interface CurrentListResult {
	type: 'CurrentListResult'
	listings: FileListing[]
}

export interface CreateFile {
	type: 'CreateFile'
	name: string
	fileType: FileType
	key: string
}

export interface RenameFile {
	type: 'RenameFile'
	name: string
	key: string
}

export interface DeleteFile {
	type: 'DeleteFile'
	key: string
}

export type FileListMessage =
	| CreateFile
	| CurrentListResult
	| DeleteFile
	| GetCurrentList
	| RenameFile
