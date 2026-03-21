<script lang="ts">
    import Navbar from "$lib/navbar.svelte";
</script>

<svelte:head>
    <meta name="title" content="Modu" />
    <meta name="description" content="A simple and fast interpreted programming language." />
</svelte:head>

<div class="flex w-full max-w-screen h-screen flex-col max-w-screen overflow-x-hidden">
    <Navbar />

    <div class="m-auto">
        <div class="flex flex-col px-2 md:mx-auto">
            <div class="mx-auto p-4 text-center my-4">
                <p class="text-xl whitespace-pre-wrap">
Modu is a simple and fast interpreted programming language. 
It is designed to be easy to learn and use like python.
But with the speed and efficiency of rust :D
With streamlined error messages for easy debugging</p>
            </div>
        </div>

        <div class="flex flex-col px-2">
            <div class="mx-auto bg-bg0_h p-4 rounded-lg border border-bg2 my-4">
                <p class="text-xl whitespace-pre-wrap font-mono"><span class="font-bold">Errors that help you, not confuse you<br/></span>
<span class="text-red">Error:</span> undefined variable 'pretn'
   ╭─[ test.modu:1:1 ]
   │
 1 │ pretn("hi");
   │ 
   │ Help: did you maybe mean: <span class="text-[#8ec07c]">'print'</span>?
───╯</p>
            </div>
        </div>
    </div>
</div>