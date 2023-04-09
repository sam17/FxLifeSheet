export interface RawDateData {
  timestamp: string;
  value: number;
}

class DateData {
  date: Date;
  value: number;

  constructor(timestamp: string, value: number) {
    this.date = new Date(timestamp);
    this.value = value;
  }
}

class ArrayDateData {
  data: Array<DateData> = [];
  maxRange: number = 0;
  minRange: number = 0; 
  isPositive: boolean = true;
  isReverse: boolean = false;

  constructor(arrayOfRawData: Array<RawDateData>, maxRange: number, minRange: number, isPositive: boolean, isReverse: boolean) {
    this.data = arrayOfRawData["data"].map(
      (d) => new DateData(d.timestamp, d.value)
    );
    this.maxRange = maxRange;
    this.minRange = minRange;
    this.isPositive = isPositive;
    this.isReverse = isReverse;
  }

  getDates(): Array<string> {
    return this.data.map((d) => d.date.toISOString().split("T")[0]);
  }

  getValue(date: Date): number {
    return this.data[this.isDateInArrayIndex(date)].value;
  }

  getValueModified(date: Date): number {
    let value = this.getAverageValueForDate(date)
    if (this.isReverse) {
        value = this.maxRange - value;
        return value + this.minRange;
    } else {
        return value;
    }
  }
  compareDates(a: Date, b: Date): boolean {
    let x = new Date(a);
    let y = new Date(b);
    x.setHours(0, 0, 0);
    y.setHours(0, 0, 0);
    return x.getTime() === y.getTime();
  }

  isDateInArray(a: Date) {
    return this.data.some((d) => this.compareDates(d.date, a));
  }

  // above function but returns the index
  isDateInArrayIndex(a: Date) {
    return this.data.findIndex((d) => this.compareDates(d.date, a));
  }

  getAllDataForDate(a: Date) {
    return this.data.filter((d) => this.compareDates(d.date, a));
  }

  getAverageValueForDate(a: Date) {
    let values = this.getAllDataForDate(a).map((d) => d.value);
    values = values.map((d) => Number(d));
    let average =  values.reduce((a, b) => a + b, 0) / values.length;
    return Math.round(average);
  }

  getArray() {
    const dataArray = this.data.map((item) => {
      return {
        date: item.date,
        value: this.getValueModified(item.date),
      };
    });

    return dataArray;
  }

  public getData(): Array<DateData> {
    return this.data;
  }
}

export { ArrayDateData, DateData}