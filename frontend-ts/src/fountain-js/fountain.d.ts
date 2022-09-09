import { Token } from './token';
export interface Script {
    title?: string;
    html: {
        title_page: string;
        script: string;
    };
    tokens?: Token[];
}
export declare class Fountain {
    private tokens;
    private scanner;
    private inlineLex;
    constructor();
    parse(script: string, getTokens?: boolean): Script;
    to_html(token: Token): string;
}
