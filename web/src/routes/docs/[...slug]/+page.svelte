<script lang="ts">
    import { onMount } from 'svelte';
    import { marked } from 'marked';
    import { Menu, MoveLeft } from 'lucide-svelte';
    import docData from '$lib/docs/data';
    import "./docs.css";
    import { base } from '$app/paths';

    export let data;
    $: {
        data.slug;
        fetchMarkdown();
    }

    async function fetchMarkdown() {
        try {
            await import(`$lib/docs/${data.slug.replace(".md", "")}.md?raw`)
                .then(async (res) => {
                    markdown = await marked(res.default);

                    setTimeout(() => {
                        if (typeof Prism !== "undefined") {
                            Prism.highlightAll();
                        }
                    }, 100);
                });
        } catch (e) {
            markdown = await marked(`# 404 - Not Found`);
        }
    }

    let markdown = "";
    let sidebarOpen = true;

    onMount(fetchMarkdown);
</script>

<svelte:head>
    <link
        href="https://rawcdn.githack.com/cyteon/assets/05f98dd21d2870fe70f1f8d51912ef0e2ee9c9c8/prismjs/css/mocha.css"
        rel="stylesheet"
    />

    <script
        src="https://rawcdn.githack.com/cyteon/assets/d294f053d31b9b61beedab38577934a6bab764d7/prismjs/js/rust-only.js"
    ></script>

    <meta name="title" content="Modu Documentation" />
    <meta name="description" content="Documentation for the Modu programming language." />
</svelte:head>

<div class="flex h-screen w-full">
    <div class={`h-screen bg-ctp-mantle p-2 border-r border-r-ctp-surface0 flex flex-col transition-all duration-300 ${sidebarOpen ? "w-52" : "w-16"}`}>
        <a href={base + "/"} class="flex text-xl font-bold transition-color duration-300 hover:bg-ctp-crust/80 p-2 rounded-md">
            <MoveLeft size={32} class="my-auto text-lg bg-ctp-base p-1 border border-ctp-surface0 rounded-md" />
            <span class={`ml-2 my-auto ${sidebarOpen ? "" : "hidden"}`}>Main Site</span>
        </a>
        {#each docData.pages as page}
            <a href={page.path} class="flex text-xl font-bold transition-color duration-300 hover:bg-ctp-crust/80 p-2 rounded-md">
                <page.icon size={32} class={`my-auto text-lg bg-ctp-base p-1 border border-ctp-surface0 rounded-md ${data.slug === page.path ? "text-ctp-blue" : ""}`} />
                <span class={`ml-2 my-auto ${data.slug === page.path ? "text-ctp-blue" : ""} ${sidebarOpen ? "" : "hidden"}`}>{page.title}</span>
            </a>
        {/each}

        <button class="mt-auto bg-ctp-mantle p-2 rounded-md" on:click={() => sidebarOpen = !sidebarOpen}>
            <Menu size={32} class="my-auto" />
        </button>
    </div>

    <div class="w-full flex overflow-auto min-h-screen flex-col">
        <p class="prose prose-lg px-4 py-4 xl:px-64 md:py-8 min-w-full">
            {@html markdown}
        </p>

        <p class="pb-7 text-lg mx-auto mt-auto">End of Page</p>
    </div>
</div>