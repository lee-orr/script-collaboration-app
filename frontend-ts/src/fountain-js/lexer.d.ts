export declare class Lexer {
    reconstruct(script: string): string;
}
export declare class InlineLexer extends Lexer {
    private inline;
    reconstruct(line: string): string;
}
