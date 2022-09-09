import { Token } from './token';
export declare class Scanner {
    private tokens;
    tokenize(script: string): Token[];
}
