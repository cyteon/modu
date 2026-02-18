import { Home, Baseline, File, FileBox, Equal, Library, AppWindowIcon, Box, Braces, Wifi, Repeat2, CaseLower, MessageSquareLock } from "lucide-svelte"

export default {
    pages: [
        {
            "path": "quickstart",
            "title": "Quickstart",
            "icon": Home,
        }, 
        {
            "path": "basics",
            "title": "Basics",
            "icon": Baseline,
        },
        {
            "path": "imports",
            "title": "Imports",
            "icon": FileBox,
        },
        {
            "path": "libraries",
            "title": "Libraries",
            "icon": Library,
        },
        {
            "path": "math",
            "title": "Math",
            "icon": Equal,
        },
        {
            "path": "file",
            "title": "File I/O",
            "icon": File,
        },
        {
            "path": "os",
            "title": "OS Lib",
            "icon": AppWindowIcon,
        },
        {
            "path": "ffi",
            "title": "FFI",
            "icon": Box,
        },
        {
            "path": "json",
            "title": "JSON",
            "icon": Braces,
        },
        {
            "path": "http",
            "title": "HTTP",
            "icon": Wifi,
        },
        {
            "path": "loops",
            "title": "Loops",
            "icon": Repeat2,
        },
        {
            "path": "encoding",
            "title": "Encoding",
            "icon": CaseLower,
        },
        {
            "path": "cryptography",
            "title": "Cryptography",
            "icon": MessageSquareLock,
        }
    ]
}