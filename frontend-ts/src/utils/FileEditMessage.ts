export interface FileContentUpdated {
	type: 'FileContentUpdated'
	key: string
	update: string
}

export interface RequestFileState {
	type: 'RequestFileState'
	key: string
}

export interface FullFileState {
	type: 'FullFileState'
	key: string
	update: string
}

export type FileEditMessage =
	| FileContentUpdated
	| RequestFileState
	| FullFileState
