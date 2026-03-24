import { marked } from 'marked';

let pages = [
    {
        title: "Getting Started",
        lessons: ["hello-world"]
    },
    {
        title: "Basics",
        lessons: ["variables", "functions", "type-conversion", "classes", "error-handling", "bitwise-ops"]
    },
    {
        title: "Types",
        lessons: ["integers", "floats", "strings", "booleans", "arrays", "objects"]
    },
    {
        title: "Control Flow",
        lessons: ["if-else", "for-loops", "while-loops", "infinite-loops", "break-continue"]
    },
    {
        title: "Standard Library",
        lessons: [
            "stdlib-intro", "stdlib-math", "stdlib-json", 
            "stdlib-time", "stdlib-uuid", "stdlib-crypto", 
            "stdlib-encoding", "stdlib-os", "stdlib-fs", 
            "stdlib-http", "stdlib-ffi"
        ]
    }
]

export async function getLesson(slug: string): Promise<{ html: string, code: string }> {
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
    let slugs = pages.flatMap(page => page.lessons);
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
    let slugs = pages.flatMap(page => page.lessons);
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