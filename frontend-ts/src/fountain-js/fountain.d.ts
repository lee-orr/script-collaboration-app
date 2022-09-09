import type { Token } from './token'

export interface Script {
	title?: string
	html: {
		title_page: string
		script: string
	}
	tokens?: Token[]
}
export declare class Fountain {
	private readonly tokens

	private readonly scanner

	private readonly inlineLex
	constructor()
	parse(script: string, getTokens?: boolean): Script
	to_html(token: Token): string
}
