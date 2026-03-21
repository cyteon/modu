import { basicSetup, EditorView } from "codemirror";
import { EditorState } from "@codemirror/state"
import { HighlightStyle, syntaxHighlighting, bracketMatching, indentUnit } from "@codemirror/language"
import { tags } from "@lezer/highlight"
import { keymap } from "@codemirror/view";
import { indentWithTab } from "@codemirror/commands";
import { closeBrackets, closeBracketsKeymap } from "@codemirror/autocomplete";
import moduSyntax from "$lib/moduSyntax.js";

export function newIDE() {
    return EditorState.create({
        doc: "print(\"hello, world!\");",
        extensions: [
            basicSetup,
            EditorState.tabSize.of(4),
            indentUnit.of("    "),
            moduSyntax,
            keymap.of([indentWithTab, ...closeBracketsKeymap]),
            closeBrackets(),
            bracketMatching(),
            EditorView.theme({
                "&": {
                    color: "#fbf1c7",
                    backgroundColor: "#1d2021",
                    fontSize: "16px",
                    height: "100%",
                    maxHeight: "100%",
                    borderRadius: "0.5rem",
                    overflow: "hidden",
                },

                "&.cm-focused": {
                    outline: "none",
                },

                ".cm-scroller": {
                    overflow: "auto",
                },

                ".cm-activeLine": {
                    backgroundColor: "transparent",
                },

                ".cm-activeLineGutter" : {
                    backgroundColor: "transparent",
                },

                ".cm-gutters": {
                    backgroundColor: "#1d2021",
                    border: "none",
                },

            }, { dark: true }),
            syntaxHighlighting(HighlightStyle.define([
                { tag: tags.string, color: "#a6e3a1" },
                { tag: tags.keyword, color: "#cba6f7" },
                { tag: tags.atom, color: "#f38ba8" },
                { tag: tags.escape, color: "#f5c2e7" },
                { tag: tags.comment, color: "#a89984" },
                { tag: tags.number, color: "#fab387" },
                { tag: tags.float, color: "#fab387" },
                { tag: tags.operator, color: "#89dceb" },
                { tag: tags.brace, color: "#a89984" },
                { tag: tags.bool, color: "#89b4fa" }
            ])),
        ]
    });
}