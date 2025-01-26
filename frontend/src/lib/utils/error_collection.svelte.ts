interface ErrorValue {
    errorText: string;
    id: number;
}

class ErrorCollection {
    private errorList: ErrorValue[] = $state([]);

    removeError(id: number) {
        this.errorList = this.errorList.filter((item) => item.id !== id);
    }

    getError(id: number): ErrorValue | undefined {
        return this.errorList.find((item) => item.id === id);
    }

    hasErrors(): boolean {
        return this.errorList.length !== 0;
    }

    addError(id: number, errorText: string) {
        this.errorList.push({ id, errorText });
    }
}

export { ErrorCollection, type ErrorValue };
