export declare class Lexer {
	reconstruct(script: string): string
}
export declare class InlineLexer extends Lexer {
	private readonly inline
	reconstruct(line: string): string
}
