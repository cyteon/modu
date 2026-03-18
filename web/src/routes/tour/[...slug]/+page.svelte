<script lang="ts">
    import { Play } from "lucide-svelte";
    import { basicSetup, EditorView } from "codemirror";
    import { EditorState, Compartment } from "@codemirror/state"
    import { onMount, onDestroy } from "svelte";

    import Navbar from '$lib/navbar.svelte';
    import { getLesson, getNext, getPrevious } from '$lib/tour/data';
    import { AnsiUp } from "ansi_up";

    let slug: string;

    let html = "";

    let language = new Compartment, tabsize = new Compartment;

    let view: EditorView | undefined;

    onMount(() => {
        const state = EditorState.create({
            doc: "",
            extensions: [
                basicSetup,
                tabsize.of(EditorState.tabSize.of(4)),
                EditorView.theme({
                    "&": {
                        color: "#fbf1c7",
                        backgroundColor: "#1d2021",
                        fontSize: "16px",
                        height: "100%",
                        borderRadius: "0.5rem",
                        overflow: "auto",
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

                }, { dark: true })
            ]
        });

        view = new EditorView({
            state,
            parent: document.getElementById("code")!,
        });
    });

    onDestroy(() => {
        view?.destroy();
    });

    var wasm;

    let currentSlug = "";

    export let data;
    $: {
        slug = data.slug;

        if (slug !== currentSlug) {
            currentSlug = slug;

            (async () => {
                const lesson = await getLesson(slug);
                html = lesson.html;

                if (view) {
                    view.dispatch({
                        changes: { from: 0, to: view.state.doc.length, insert: lesson.code },
                    });
                }

                if (!wasm) {
                    wasm = await import("modu-wasm");
                    await wasm.init();

                    run(); // run code on load
                }

                console.log(lesson);
            })();
        }
    }

    let output = "";

    let ansi = new AnsiUp();
    ansi.use_classes = true;

    async function run() {
        try {
            const code = view?.state.doc.toString() || "";
            const result = await wasm.eval_modu(code);
            output = ansi.ansi_to_html(result);
        } catch (e) {
            output = `Error: ${e}`;
        }
    }
</script>

<div class="flex w-full max-w-screen h-screen flex-col max-w-screen overflow-x-hidden">
    <Navbar />

    <hr class="border-bg2 mt-1" />

    <div class="flex flex-col md:flex-row p-4 md:space-x-8 space-y-6 md:space-y-0 flex-1">
        <div class="md:w-1/2 flex flex-col">
            <div class="prose max-w-none border border-bg2 py-2 px-4 rounded-lg flex-1">{@html html}</div>
            <div class="mt-6 text-center">
                <a href={getPrevious(slug)} class={`text-blue hover:underline ${!getPrevious(slug) && "opacity-50 cursor-not-allowed"}`}>&lt; previous</a>
                <span class="mx-2 text-fg2">—</span>
                <a href={getNext(slug)} class={`text-blue hover:underline ${!getNext(slug) && "opacity-50 cursor-not-allowed"}`}>next &gt;</a>
            </div>
        </div>

        <div class="flex flex-col h-full md:w-1/2 space-y-2">
            <div class="h-2/3 relative">
                <div class="h-full border border-bg2 rounded-lg p-1 bg-bg0_h" id="code"></div>
                <button class="absolute top-4 right-4" on:click={run}>
                    <Play size="20" />
                </button>
            </div>
            <pre class="h-1/3 bg-bg0_h py-2 px-4 rounded-lg border border-bg2 text-lg overflow-auto whitespace-pre-wrap">{@html output}</pre>
        </div>
    </div>
</div>