<script lang="ts">
    import { Github, ExternalLink, Book, CircleHelp, Box, Code } from "lucide-svelte";
    import { base } from "$app/paths";

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
    <link
        href="https://rawcdn.githack.com/cyteon/assets/05f98dd21d2870fe70f1f8d51912ef0e2ee9c9c8/prismjs/css/mocha.css"
        rel="stylesheet"
    />

    <script
        src="https://rawcdn.githack.com/cyteon/assets/d294f053d31b9b61beedab38577934a6bab764d7/prismjs/js/rust-only.js"
    ></script>

    <meta name="title" content="Modu" />
    <meta name="description" content="A simple and fast interpreted programming language." />
</svelte:head>

<div class="flex w-full max-w-screen h-screen flex-col max-w-screen overflow-x-hidden">
    <div class="h-fit bg-ctp-mantle text-center pt-2 pb-6 border-b border-b-ctp-surface0">
        <h1 class="text-8xl font-bold inline-block bg-clip-text bg-gradient-to-r from-ctp-red to-ctp-yellow to-75% text-transparent mb-4">Modu</h1>
        
        <div class="flex space-x-2 justify-center">
            <a 
            class="flex w-fit bg-white p-2 rounded-md text-ctp-mantle transition-all duration-300" 
            href="https://github.com/cyteon/modu" target="_blank" 
            on:mouseenter={() => githubHovered = true} 
            on:mouseleave={() => githubHovered = false}
        >
            <Github size={24} class="my-auto" />
            <span class="ml-1 text-xl">GitHub</span>
            <ExternalLink size={20} class={`my-auto  ${githubHovered ? "w-6 ml-1" : "w-0"} transition-all duration-300`} />
        </a>

        <a 
            class="flex w-fit bg-ctp-peach p-2 rounded-md text-ctp-mantle transition-all duration-300" 
            href="https://crates.io/crates/modu" target="_blank" 
            on:mouseenter={() => cratesIoHovered = true} 
            on:mouseleave={() => cratesIoHovered = false}
        >
            <Box size={24} class="my-auto" />
            <span class="ml-1 text-xl">Crates.io</span>
            <ExternalLink size={20} class={`my-auto  ${cratesIoHovered ? "w-6 ml-1" : "w-0"} transition-all duration-300`} />
        </a>
        </div>
    </div>

    <div class="flex flex-col px-2 md:mx-auto">
        <div class="my-6 mx-auto space-y-2 flex-col flex md:flex-row md:space-x-4 md:space-y-0">
            <a 
                href={base + "/examples"}
                class="bg-ctp-blue text-ctp-crust text-xl p-2 rounded-md text-center w-full flex md:w-fit"
                on:mouseenter={() => moreExamplesHovered = true}
                on:mouseleave={() => moreExamplesHovered = false}
            >
                <CircleHelp size={20} class="my-auto" />
                <span class="ml-2">More Examples</span>
                <ExternalLink size={20} class={`my-auto ${moreExamplesHovered ? "w-6 ml-1" : "w-0"} transition-all duration-300`} />
            </a>

            <a 
                href={base + "/docs/quickstart"}
                class="bg-ctp-pink text-ctp-crust text-xl p-2 rounded-md text-center w-full flex md:w-fit"
                on:mouseenter={() => docsHovered = true}
                on:mouseleave={() => docsHovered = false}
            >
                <Book size={20} class="my-auto" />
                <span class="ml-2">Docs</span>
                <ExternalLink size={20} class={`my-auto ${docsHovered ? "w-6 ml-1" : "w-0"} transition-all duration-300`} />
            </a>

            <a 
                href={base + "/ide"}
                class="bg-ctp-yellow text-ctp-crust text-xl p-2 rounded-md text-center w-full flex md:w-fit"
                on:mouseenter={() => ideHovered = true}
                on:mouseleave={() => ideHovered = false}
            >
                <Code size={20} class="my-auto" />
                <span class="ml-2">Web IDE</span>
                <ExternalLink size={20} class={`my-auto ${ideHovered ? "w-6 ml-1" : "w-0"} transition-all duration-300`} />
            </a>
        </div>

        <div class="text- mx-auto bg-ctp-mantle p-4 rounded-md">
            <p class="text-xl whitespace-pre-wrap">
Modu is a simple and fast interpreted programming language. 
It is designed to be easy to learn and use like python.
But with the speed and efficiency of rust :D
With streamlined error messages for easy debugging</p>
        </div>

        <div class="mx-auto bg-ctp-mantle py-3 w-full rounded-md mt-4 mb-4 max-w-screen">
            <h1 class="text-3xl font-bold text-center">Example</h1>
            <pre class="text-lg whitespace-pre-wrap break-words language-rust"><code>{`let a = 1;
let b = -2;
let c = a - b;
                        
if a == b {
    print("a is equal to b");
}
                        
                        
if a != b {
    print("a is not equal to b");
}
                        
fn example() {
    print("Hello, World");
}
                        
if c == 3 {
    example();
}
                        
                        
// Output
// 
// a is not equal to b
// Hello, World!`}</code></pre>
        </div>
    </div>
</div>