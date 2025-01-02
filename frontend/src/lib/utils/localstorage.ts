export function getWithFallback(key: string, fallback: string): string {
    let item = localStorage.getItem(key);
    if (item === null) localStorage.setItem(key, fallback);
    return item || fallback;
}
