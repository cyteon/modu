import { marked } from 'marked';

let pages = [
    {
        title: "Getting Started",
        lessons: [
            { slug: "hello-world", title: "Hello World" },
        ],
    },
    {
        title: "Basics",
        lessons: [
            { slug: "variables", title: "Variables" },
            { slug: "functions", title: "Functions" },
            { slug: "type-conversion", title: "Type Conversion" }
        ]
    },
    {
        title: "Types",
        lessons: [
            { slug: "integers", title: "Integers" },
            { slug: "floats", title: "Floats" },
            { slug: "strings", title: "Strings" },
            { slug: "booleans", title: "Booleans" },
            { slug: "arrays", title: "Arrays" },
            { slug: "objects", title: "Objects / JSON" },
        ]
    },
    {
        title: "Control Flow",
        lessons: [
            { slug: "if-else", title: "If / Else" },
            { slug: "for-loops", title: "For Loops" },
            { slug: "while-loops", title: "While Loops" },
            { slug: "infinite-loops", title: "Infinite Loops" },
            { slug: "break-continue", title: "Break / Continue" },
        ]
    },
    {
        title: "Standard Library",
        lessons: [
            { slug: "stdlib-intro", title: "Introduction" },
            { slug: "stdlib-math", title: "Math" },
            { slug: "stdlib-json", title: "JSON" },
            { slug: "stdlib-time", title: "Time" },
            { slug: "stdlib-uuid", title: "UUID"},
            { slug: "stdlib-crypto", title: "Cryptography" },
            { slug: "stdlib-encoding", title: "Encoding" },
            { slug: "stdlib-os", title: "OS" },
            { slug: "stdlib-fs", title: "Filesystem" },
            { slug: "stdlib-http", title: "HTTP" },
            { slug: "stdlib-ffi", title: "FFI" }
        ]
    }
]

export async function getLesson(slug: string): { html: string, code: string } {
    try {
        let md = (await import(`$lib/tour/pages/${slug.replace(".md", "")}.md?raw`)).default // so hello-world and hello-world.md both work
        let [docs, code] = md.split("[CODE]");
        let html = await marked(docs.trim());
        return { html, code: code.trim() };
    } catch (e) {
        console.error(`Failed to load lesson ${slug}:`, e);
        return { html: "<p>Failed to load lesson.</p>", code: "" };
    }
}

export function getPrevious(currentSlug: string): string | null {
    let slugs = pages.flatMap(page => page.lessons.map(lesson => lesson.slug));
    let currentIndex = slugs.indexOf(currentSlug);

    if (currentIndex === -1) {
        return slugs[0];
    }

    if (currentIndex > 0) {
        return slugs[currentIndex - 1];
    } else {
        return null;
    }
}

export function getNext(currentSlug: string): string | null {
    let slugs = pages.flatMap(page => page.lessons.map(lesson => lesson.slug));
    let currentIndex = slugs.indexOf(currentSlug);

    if (currentIndex === -1) {
        return slugs[0];
    }

    if (currentIndex < slugs.length - 1) {
        return slugs[currentIndex + 1];
    } else {
        return null;
    }
}

export default pages;