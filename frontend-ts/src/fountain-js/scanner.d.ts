import type { Token } from './token'

export declare class Scanner {
	private readonly tokens
	tokenize(script: string): Token[]
}
