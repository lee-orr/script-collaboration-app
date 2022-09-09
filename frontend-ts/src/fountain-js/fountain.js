import { Scanner } from './scanner';
import { InlineLexer } from './lexer';
export class Fountain {
    constructor() {
        this.scanner = new Scanner;
        this.inlineLex = new InlineLexer;
    }
    parse(script, getTokens) {
        // throw an error if given script source isn't a string
        if (typeof script === 'undefined' || script === null)
            throw new Error("Script is undefined or null.");
        if (typeof script !== 'string')
            throw new Error(`Script should be \`string\`, input was \`${Object.prototype.toString.call(script)}\`.`);
        try {
            this.tokens = this.scanner.tokenize(script);
            let title = this.tokens.find(token => token.type === 'title');
            return {
                title: title ? this.inlineLex.reconstruct(title.text)
                    .replace('<br />', ' ').replace(/<(?:.|\n)*?>/g, '') : undefined,
                html: {
                    title_page: this.tokens.filter(token => token.is_title).map(token => this.to_html(token)).join(''),
                    script: this.tokens.filter(token => !token.is_title).map(token => this.to_html(token)).join('')
                },
                tokens: getTokens ? this.tokens : undefined
            };
        }
        catch (error) {
            error.message +=
                '\nPlease submit an issue to https://github.com/jonnygreenwald/fountain-js/issues';
            throw error;
        }
    }
    to_html(token) {
        token.text = this.inlineLex.reconstruct(token.text);
        switch (token.type) {
            case 'title': return '<h1>' + token.text + '</h1>';
            case 'credit': return '<p class="credit">' + token.text + '</p>';
            case 'author': return '<p class="authors">' + token.text + '</p>';
            case 'authors': return '<p class="authors">' + token.text + '</p>';
            case 'source': return '<p class="source">' + token.text + '</p>';
            case 'notes': return '<p class="notes">' + token.text + '</p>';
            case 'draft_date': return '<p class="draft-date">' + token.text + '</p>';
            case 'date': return '<p class="date">' + token.text + '</p>';
            case 'contact': return '<p class="contact">' + token.text + '</p>';
            case 'copyright': return '<p class="copyright">' + token.text + '</p>';
            case 'scene_heading': return '<h3' + (token.scene_number ? ' id="' + token.scene_number + '">' : '>') + token.text + '</h3>';
            case 'transition': return '<h2>' + token.text + '</h2>';
            case 'dual_dialogue_begin': return '<div class="dual-dialogue">';
            case 'dialogue_begin': return '<div class="dialogue' + (token.dual ? ' ' + token.dual : '') + '">';
            case 'character': return '<h4>' + token.text + '</h4>';
            case 'parenthetical': return '<p class="parenthetical">' + token.text + '</p>';
            case 'dialogue': return '<p>' + token.text + '</p>';
            case 'dialogue_end': return '</div>';
            case 'dual_dialogue_end': return '</div>';
            case 'section': return '<p class="section" data-depth="' + token.depth + '">' + token.text + '</p>';
            case 'synopsis': return '<p class="synopsis">' + token.text + '</p>';
            case 'note': return '<!-- ' + token.text + ' -->';
            case 'boneyard_begin': return '<!-- ';
            case 'boneyard_end': return ' -->';
            case 'action': return '<p>' + token.text + '</p>';
            case 'centered': return '<p class="centered">' + token.text + '</p>';
            case 'lyrics': return '<p class="lyrics">' + token.text + '</p>';
            case 'page_break': return '<hr />';
            case 'line_break': return '<br />';
        }
    }
}
//# sourceMappingURL=fountain.js.map