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
			event: 'open' | 'error' | 'close',
			error?: Error
		) => void
	) => void
	removeListener: (callback: (message: T) => void) => void
	close: () => void
}

const ONE = 1

export function createPeerNetworkAdapter<T>(): NetworkAdapter<T> {
	const key = GenerateKey().replaceAll('-', '').substring(0, 8)
	const id = `${key}`
	const peer: Peer = new Peer(id, {
		config: {
			iceServers: [{ urls: 'stun:stun.l.google.com:19302' }],
			sdpSemantics: 'unified-plan'
		}
	})

	let listeners: ((message: T) => void)[] = []
	let connectionEventListener:
		| undefined
		| ((
				connection: DataConnection,
				event: 'open' | 'error' | 'close',
				error?: Error
		  ) => void) = undefined

	const connections: DataConnection[] = []

	peer.on('connection', connection => {
		connections.push(connection)
		connection.on('data', data => {
			console.log('Got Message: ', data)
			for (const listener of listeners) {
				listener(data as T)
			}
		})
		connection.on('open', () => {
			console.log('Opened connection', connection)
			if (connectionEventListener) connectionEventListener(connection, 'open')
		})
		connection.on('close', () => {
			connections.splice(connections.indexOf(connection), ONE)
			if (connectionEventListener) connectionEventListener(connection, 'close')
		})
		connection.on('error', e => {
			console.error('Connection Error', e)
			if (connectionEventListener)
				connectionEventListener(connection, 'error', e)
		})
	})

	let currentRemote: string = ''

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
					console.log('Got Message: ', data)
					for (const listener of listeners) {
						listener(data as T)
					}
				})
				connection.on('open', () => {
					console.log('Opened connection', connection)
					if (connectionEventListener)
						connectionEventListener(connection, 'open')
				})
				connection.on('close', () => {
					connections.splice(connections.indexOf(connection), ONE)
					if (connectionEventListener)
						connectionEventListener(connection, 'close')
				})
				connection.on('error', e => {
					console.error('Connection Error', e)
					if (connectionEventListener)
						connectionEventListener(connection, 'error', e)
				})
			}, 1000)
			return key
		},
		sendMessage(message): void {
			console.log('Sending Message: ', message)
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
