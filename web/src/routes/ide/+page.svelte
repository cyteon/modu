<script lang="ts">
    import { Play, Download, Upload } from "lucide-svelte";
    import { basicSetup, EditorView } from "codemirror";
    import { EditorState, Compartment } from "@codemirror/state"
    import { rust } from "@codemirror/lang-rust";
    import {tags} from "@lezer/highlight"
    import { HighlightStyle, syntaxHighlighting } from "@codemirror/language"
    import { browser } from "$app/environment";
    import { onMount } from "svelte";
    import { base } from "$app/paths";

    import init, { eval_modu, modu_version } from "$lib/modu/modu_wasm.js";

    let language = new Compartment, tabsize = new Compartment;
    let moduVersion = "";

    let state = EditorState.create({
        doc: `fn yap(str) {
    print(str);
}

yap("Hello, World!");

// Expected Output:
//
// Hello, World!
`,
        extensions: [
            basicSetup,
            language.of(rust()),
            tabsize.of(EditorState.tabSize.of(4)),
            EditorView.theme({
                "&": {
                    color: "#fbg1c7",
                    backgroundColor: "#282828",
                    fontSize: "24px",
                    height: "100%",
                },

                "&.cm-focused": {
                    outline: "none",
                },

                ".cm-activeLine": {
                    backgroundColor: "#1d202180",
                },

                ".cm-activeLineGutter" : {
                    backgroundColor: "#28282880",
                },

                ".cm-gutters": {
                    backgroundColor: "#1d2021",
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

    let view;

    onMount(async () => {
        if (browser) {
            await init();
            moduVersion = modu_version();

            view = new EditorView({
                state,
                parent: document.querySelector("#code"),
            });
        }
    });

    let output = "Run the code to see the output";
    let runClicked = false;

    const ansiRegex = /\x1b\[[0-9;]*m/g;

    async function run() {
        try {
            runClicked = true;

            const code = view.state.doc.toString();

            if (code.includes("import \"http\"")) {
                output = "Error running code: The HTTP client does currently not work on the web";
                runClicked = false;
                return;
            } else if (code.includes("import \"ffi\"")) {
                output = "Error running code: Using FFI is not possible on the web";
                runClicked = false;
                return;
            } else if (code.includes("import \"os\"")) {
                output = "Error running code: Using the OS module is not supported in the web IDE cause its running on the web";
                runClicked = false;
                return;
            }

            let result = eval_modu(code);
        
            output = result.replace(ansiRegex, "");

            setTimeout(() => {
                runClicked = false;
            }, 1000);
        } catch (e) {
            output = "Error running code: " + e.message;
        }
    }

    function download() {
        const blob = new Blob([view.state.doc.toString()], { type: "text/plain" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "main.modu";
        a.click();
    }

    function upload() {
        const input = document.createElement("input");
        input.type = "file";
        input.accept = ".modu";

        input.onchange = async () => {
            const file = input.files[0];
            const text = await file.text();
            view.dispatch({
                changes: { from: 0, to: view.state.doc.length, insert: text },
            });
        };

        input.click();
    }
</script>

<svelte:head>
    <meta name="title" content="Modu Web IDE" />
    <meta name="description" content="Online IDE for Modu, run code without installing anything." />
</svelte:head>

<div class="flex flex-col w-full h-screen">
    <div class="w-full border-b border-bg1 p-2 px-4 flex">
        <a href={base + "/"} class="text-3xl font-bold inline-block bg-clip-text text-transparent bg-gradient-to-r from-ctp-red to-75% to-ctp-yellow">Modu</a>
        <p class="ml-2 mt-auto text-xl">{moduVersion ? `${moduVersion}` : ""}</p>

        <div class="ml-auto my-auto">
            <a href="docs" class="text-2xl">Docs</a>
        </div>

        <div class="ml-auto flex">
            <button class={`${runClicked ? "text-ctp-green" : ""} mr-5`} on:click={run}>
                <Play size={28} class="my-auto" />
            </button>

            <button class="mr-5" on:click={upload}>
                <Download size={28} class="my-auto" />
            </button>

            <button on:click={download}>
                <Upload size={28} class="my-auto" />
            </button>
        </div>
    </div>

    <div class="flex p-4 h-full space-y-4 flex-col md:flex-row md:space-x-4 md:space-y-0">
        <div class="bg-bg1 w-full p-6 pt-4 h-full rounded-md flex flex-col md:w-2/3 border border-bg2">
            <h1 class="text-3xl font-bold">Input</h1>
            <div id="code" class="mt-4 h-full max-h-[83vh] rounded-md"></div>
        </div>

        <div class="bg-bg1 w-full p-6 pt-4 h-full rounded-md flex flex-col md:w-1/3 border border-bg2">
            <h1 class="text-3xl font-bold">Output</h1>
            <pre class="px-4 py-2 mt-4 text-xl break-words whitespace-pre-wrap bg-bg h-full">{output}</pre>
        </div>
    </div>
</div>

<style>
    button {
        @apply rounded-md my-auto text-center font-mono w-fit flex transition-all duration-300 hover:text-blue;
    }
</style>