<script lang="ts">
    import { Github, ExternalLink, Book, CircleHelp, Box, Code } from "lucide-svelte";
    import { base } from "$app/paths";

    import Navbar from "$lib/navbar.svelte";

    let githubHovered = false;
    let cratesIoHovered = false;
    let moreExamplesHovered = false;
    let docsHovered = false;
    let ideHovered = false;

    setTimeout(() => {
        if (typeof Prism !== "undefined") {
            Prism.highlightAll();
        }
    }, 100);
</script>

<svelte:head>
    <meta name="title" content="Modu" />
    <meta name="description" content="A simple and fast interpreted programming language." />
</svelte:head>

<div class="flex w-full max-w-screen h-screen flex-col max-w-screen overflow-x-hidden">
    <Navbar />

    <div class="m-auto">
        <div class="flex flex-col px-2 md:mx-auto">
            <div class="mx-auto bg-bg1 p-4 rounded-md border border-bg2 my-4">
                <p class="text-xl whitespace-pre-wrap">
Modu is a simple and fast interpreted programming language. 
It is designed to be easy to learn and use like python.
But with the speed and efficiency of rust :D
With streamlined error messages for easy debugging</p>
            </div>
        </div>

        <div class="flex flex-col px-2">
            <div class="mx-auto bg-bg1 p-4 rounded-md border border-bg2 my-4">
                <p class="text-xl whitespace-pre-wrap font-mono"><span class="font-bold">Example Error:<br/></span>
<span class="text-red">[00] Error:</span> Lexing error: UnexpectedToken
    ╭─[ examples/test.modu:1:7 ]
    │
  1 │ print(<span class="text-red">!</span>);
    │       ┬
    │       ╰── Unexpected token
────╯</p>
            </div>
        </div>
    </div>
</div>