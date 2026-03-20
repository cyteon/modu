<script lang="ts">
    import { Play, Download, Upload } from "lucide-svelte";
    import { EditorView } from "codemirror";
    import { browser } from "$app/environment";
    import { onMount } from "svelte";
    import { base } from "$app/paths";
    import { AnsiUp } from "ansi_up";
    import { newIDE } from "$lib/ide";

    let moduVersion = "";

    let state = newIDE();

    let view: EditorView | undefined;
    var wasm: any;

    onMount(async () => {
        if (browser) {
            wasm = await import("modu-wasm");

            await wasm.init();
            moduVersion = wasm.modu_version();

            view = new EditorView({
                state,
                parent: document.querySelector("#code"),
            });
        }
    });

    let output = "Run the code to see the output";
    let runClicked = false;

    let ansi = new AnsiUp();
    ansi.use_classes = true;

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

            let result = wasm.eval_modu(code);
            output = ansi.ansi_to_html(result);

            setTimeout(() => {
                runClicked = false;
            }, 500);
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
    <div class="w-full border-b border-bg1 py-1 px-6 flex">
        <a href={base + "/"} class="text-2xl font-bold">modu</a>
        <p class="ml-2 mt-auto">{moduVersion ? `v${moduVersion}` : "loading..."}</p>

        <div class="ml-auto my-auto">
            <a href="docs" class="text-lg">docs</a>
        </div>

        <div class="ml-auto flex">
            <button class={`${runClicked ? "text-blue" : ""} mr-5`} on:click={run}>
                <Play size={22} class="my-auto" />
            </button>

            <button class="mr-5" on:click={upload}>
                <Download size={22} class="my-auto" />
            </button>

            <button on:click={download}>
                <Upload size={22} class="my-auto" />
            </button>
        </div>
    </div>

    <div class="flex p-4 h-full space-y-4 flex-col md:flex-row md:space-x-4 md:space-y-0">
        <div class="bg-bg1 w-full p-6 pt-4 h-full rounded-md flex flex-col md:w-2/3 border border-bg2">
            <h1 class="text-3xl font-bold">Input</h1>
            <div id="code" class="mt-4 p-1 bg-bg0_h h-full max-h-[83vh] rounded-lg border border-bg2"></div>
        </div>

        <div class="bg-bg1 w-full p-6 pt-4 h-full rounded-md flex flex-col md:w-1/3 border border-bg2">
            <h1 class="text-3xl font-bold">Output</h1>
            <pre class="px-4 py-2 mt-4 text-xl break-words whitespace-pre-wrap bg-bg0_h border border-bg2 rounded-lg h-full overflow-y-auto">{@html output}</pre>
        </div>
    </div>
</div>

<style>
    button {
        @apply rounded-md my-auto text-center font-mono w-fit flex transition-all duration-300 hover:text-blue;
    }
</style>