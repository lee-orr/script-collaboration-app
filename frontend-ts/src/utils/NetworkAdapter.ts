import type { DataConnection } from 'peerjs'
import { Peer } from 'peerjs'
import { GenerateKey } from './KeyGenerator'

export interface NetworkAdapter<T> {
	startHost: () => string
	startClient: (remote: string) => string
	sendMessage: (message: T) => void
	setListener: (callback: (message: T) => void) => void
	connectionEventListener: (
		callback: (
			connection: DataConnection,
			event: 'close' | 'error' | 'open',
			error?: Error
		) => void
	) => void
	removeListener: (callback: (message: T) => void) => void
	close: () => void
}

const ONE = 1
const CONNECTION_TIMEOUT = 1000
const UUID_LEN = 6

export function createPeerNetworkAdapter<T>(): NetworkAdapter<T> {
	const key = GenerateKey().replaceAll('-', '').slice(0, UUID_LEN)
	const id = `${key}`
	const peer: Peer = new Peer(id)

	let listeners: ((message: T) => void)[] = []
	let connectionEventListener:
		| ((
				connection: DataConnection,
				event: 'close' | 'error' | 'open',
				error?: Error
		  ) => void)
		| undefined

	const connections: DataConnection[] = []

	peer.on('connection', connection => {
		connections.push(connection)
		connection.on('data', data => {
			for (const listener of listeners) {
				listener(data as T)
			}
		})
		connection.on('open', () => {
			if (connectionEventListener) connectionEventListener(connection, 'open')
		})
		connection.on('close', () => {
			connections.splice(connections.indexOf(connection), ONE)
			if (connectionEventListener) connectionEventListener(connection, 'close')
		})
		connection.on('error', error => {
			if (connectionEventListener)
				connectionEventListener(connection, 'error', error)
		})
	})

	let currentRemote = ''

	return {
		startHost(): string {
			return key
		},
		startClient(remote): string {
			if (currentRemote === remote) return key
			currentRemote = remote
			setTimeout(() => {
				const connection = peer.connect(`${remote}`)
				connections.push(connection)
				connection.on('data', data => {
					for (const listener of listeners) {
						listener(data as T)
					}
				})
				connection.on('open', () => {
					if (connectionEventListener)
						connectionEventListener(connection, 'open')
				})
				connection.on('close', () => {
					connections.splice(connections.indexOf(connection), ONE)
					if (connectionEventListener)
						connectionEventListener(connection, 'close')
				})
				connection.on('error', error => {
					if (connectionEventListener)
						connectionEventListener(connection, 'error', error)
				})
			}, CONNECTION_TIMEOUT)
			return key
		},
		sendMessage(message): void {
			for (const connection of connections) {
				if (connection.open) {
					connection.send(message)
				}
			}
		},
		connectionEventListener(callback): void {
			connectionEventListener = callback
		},
		setListener(callback): void {
			listeners.push(callback)
		},
		removeListener(callback): void {
			const update = []
			for (const listener of listeners) {
				if (callback !== listener) {
					update.push(listener)
				}
			}
			listeners = update
		},
		close(): void {
			peer.disconnect()
			for (const connection of connections) {
				connection.close()
			}
			connections.splice(0, connections.length)
		}
	}
}
