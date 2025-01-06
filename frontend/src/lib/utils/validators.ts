export function isEmailValid(email: string): boolean {
    return (
        String(email)
            .toLowerCase()
            .match(/^[^\s@]+@[^\s@]+\.[^\s@]+$/) != null
    );
}
