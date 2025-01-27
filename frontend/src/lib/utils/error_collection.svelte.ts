import { SvelteMap } from 'svelte/reactivity';

class ErrorCollection {
    public errorMap: SvelteMap<number, string> = new SvelteMap();

    removeError(id: number) {
        // this.errorList = this.errorList.filter((item) => item.id !== id);
        this.errorMap.delete(id);
    }

    getError(id: number): string | undefined {
        // return this.errorList.find((item) => item.id === id);
        return this.errorMap.get(id);
    }

    hasErrors(): boolean {
        return this.errorMap.size !== 0;
    }

    addError(id: number, errorText: string) {
        this.errorMap.set(id, errorText);
    }
}

function refreshError(item: string[], id: number, errorText: string, collection: ErrorCollection) {
    if (item[0] === '') collection.addError(id, errorText);
    else collection.removeError(id);
}

export { ErrorCollection, refreshError };
