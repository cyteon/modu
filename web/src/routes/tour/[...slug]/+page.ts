export const prerender = false;

export function load({ params }) {
    return {
        slug: params.slug,
    }
}