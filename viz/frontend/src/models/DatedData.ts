
export class DatedData {
    date: Date;
    value: number;

    constructor(timestamp: string, value: number) {
        // Convert the timestamp to a Date object with date only
        // this.date = new Date(timestamp).getDate();
        this.date = new Date(timestamp);
        this.value = value;
    }
}

export class DMYData {
    date: string;
    value: number;

    constructor(timestamp: string, value: number) {
        // Convert the timestamp to a Date object with date only
        this.date = new Date(timestamp).toISOString().split("T")[0];
        this.value = value;
    }
}
