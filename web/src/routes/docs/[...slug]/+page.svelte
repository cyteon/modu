<script lang="ts">
    import { onMount } from 'svelte';
    import { marked } from 'marked';
    import { Menu, MoveLeft } from 'lucide-svelte';
    import { base } from '$app/paths';

    import docData from '$lib/docs/data';

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
        href="https://rawcdn.githack.com/cyteon/assets/522e55c65c77fe6e277a51fc57b0a5053655c1a5/prismjs/css/gruvbox.css"
        rel="stylesheet"
    />

    <script
        src="https://rawcdn.githack.com/cyteon/assets/d294f053d31b9b61beedab38577934a6bab764d7/prismjs/js/rust-only.js"
    ></script>

    <meta name="title" content="Modu Docs" />
    <meta name="description" content="Docs for the Modu programming language" />
</svelte:head>

<div class="flex h-screen w-full docs">
    <div class={`h-screen overflow-y-auto bg-bg0_h p-2 border-r border-r-bg1 flex flex-col transition-all duration-300 ${sidebarOpen ? "w-64" : "w-fit"}`}>
        <a href={base + "/"} class="flex text-xl font-bold transition-color duration-300 hover:bg-bg1/80 p-2 rounded-md">
            <MoveLeft size={32} class="flex-shrink-0 my-auto text-lg bg-bg0 p-1 border border-bg1 rounded-md" />
            <span class={`ml-2 my-auto ${sidebarOpen ? "" : "hidden"}`}>Main Site</span>
        </a>

        {#each docData.pages as page}
            <a href={page.path} class="flex text-xl font-bold transition-color duration-300 hover:bg-bg1/80 p-2 rounded-md">
                <page.icon size={32} class={`flex-shrink-0 my-auto text-lg bg-bg0 p-1 border border-bg1 rounded-md ${data.slug === page.path ? "text-blue" : ""}`} />
                <span class={`ml-2 my-auto ${data.slug === page.path ? "text-blue" : ""} ${sidebarOpen ? "" : "hidden"}`}>{page.title}</span>
            </a>
        {/each}

        <button class="mt-auto p-2 rounded-md" on:click={() => sidebarOpen = !sidebarOpen}>
            <Menu size={32} class="flex-shrink-0 my-auto" />
        </button>
    </div>

    <div class="w-full flex overflow-auto min-h-screen flex-col">
        <p class="prose prose-lg px-4 py-4 xl:px-64 md:py-8 min-w-full">
            {@html markdown}
        </p>

        <p class="pb-7 text-lg mx-auto mt-auto">End of Page</p>
    </div>
</div>