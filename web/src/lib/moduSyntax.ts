import { StreamLanguage, StringStream } from "@codemirror/language";

export default StreamLanguage.define({
    startState() {
        return {
            inString: false,
            inComment: false,
        };
    },
 
    token(stream: StringStream, state: any): string | null {
        if (stream.eatSpace()) return null;

        if (stream.match("//")) {
            stream.skipToEnd();
            return "comment";
        }

        if (stream.match('"')) {
            while (!stream.eol()) {
                const ch = stream.next();
                if (ch === "\\") {
                    stream.next();
                } else if (ch === '"') {
                    break;
                }
            }

            return "string";
        }

        if (stream.match(/^[0-9]+\.[0-9]+/)) return "number";
        if (stream.match(/^[0-9]+/)) return "number";

        if (stream.match("..=") || stream.match("..")) return "operator";
        if (stream.match("+=") || stream.match("-=") || stream.match("*=") || stream.match("/=") || stream.match("%=")) return "operator";
        if (stream.match("==") || stream.match("!=") || stream.match(">=") || stream.match("<=")) return "operator";
        if (stream.match(/^[+\-*/%=<>!]/)) return "operator";

        if (stream.match(/^[{}\[\]()]/)) return "bracket";
        if (stream.match(/^[;,.]/)) return "punctuation";

        if (stream.match(/^[a-zA-Z_][a-zA-Z0-9_]*/)) {
            const word = stream.current();
 
            const keywords = [
                "let", "fn", "class", "import",
                "if", "else", "for", "while", "loop", "in",
                "return", "break", "continue"
            ];
 
            if (keywords.includes(word)) return "keyword";
            if (word === "true" || word === "false") return "bool";
            if (word === "null") return "atom";
            if (word === "self") return "variableName.special";
 
            return "variableName";
        }

        stream.next();
        return null;
    }
});