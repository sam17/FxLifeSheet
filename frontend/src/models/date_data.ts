
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
  maxRange: number;
  minRange: number;
  isPositive: boolean;
  isReverse: boolean;

  constructor(arrayOfRawData: Array<RawDateData>, maxRange: number, minRange: number, isPositive: boolean, isReverse: boolean) {
    arrayOfRawData.forEach((d) => {
      this.data.push(new DateData(d.timestamp, d.value));
    });
    this.maxRange = maxRange;
    this.minRange = minRange;
    this.isPositive = isPositive;
    this.isReverse = isReverse;
  }

  getDates(): Array<string> {
    return this.data.map((d) => d.date.toISOString().split("T")[0]);
  }

  getValue(date: Date): number {
    const index = this.findDateIndex(date);
    return this.data[index].value;
  }

  getModifiedValue(date: Date): number {
    const value = this.getAverageValueForDate(date);
    if (this.isReverse) {
      return this.maxRange - value + this.minRange;
    } else {
      return value;
    }
  }

  areDatesEqual(a: Date, b: Date): boolean {
    const x = new Date(a);
    const y = new Date(b);
    x.setHours(0, 0, 0);
    y.setHours(0, 0, 0);
    return x.getTime() === y.getTime();
  }

  dateExistsInArray(a: Date) {
    return this.data.some((d) => this.areDatesEqual(d.date, a));
  }

  findDateIndex(a: Date) {
    return this.data.findIndex((d) => this.areDatesEqual(d.date, a));
  }

  getAllDataForDate(a: Date) {
    return this.data.filter((d) => this.areDatesEqual(d.date, a));
  }

  getAverageValueForDate(a: Date) {
    let values = this.getAllDataForDate(a).map((d) => d.value);
    values = values.map((d) => Number(d));
    let average =  values.reduce((a, b) => a + b, 0) / values.length;
    return Math.round(average);
  }

  getModifiedDataArray() {
    return this.data.map((item) => ({
      date: item.date,
      value: this.getModifiedValue(item.date),
    }));
  }

  getData(): Array<DateData> {
    return this.data;
  }

  getValueInWeekOfDate(d: Date): { date: Date | null; value: number | null } {
    const targetWeekStart = this.getWeekStart(d);

    const foundDateData = this.data.find((dateData) => {
      const dateWeekStart = this.getWeekStart(dateData.date);
      return this.areDatesEqual(targetWeekStart, dateWeekStart);
    });

    if (foundDateData) {
      return { date: foundDateData.date, value: foundDateData.value };
    } else {
      return { date: null, value: null };
    }
  }

  // private getWeekStart(d: Date): Date {
  //   const date = new Date(d);
  //   const day = date.getDay();
  //   const diff = date.getDate() - day + (day === 0 ? -6 : day === 1 ? 0 : 1); // adjust for Monday
  //   const weekStart = new Date(date.setDate(diff));
  //   weekStart.setHours(0, 0, 0, 0);
  //   return weekStart;
  // }

  private getWeekStart(d: Date): Date {
    const date = new Date(d);
    const day = date.getDay();
    const diff = date.getDate() - day + (day === 0 ? -6 : 1); // adjust for Sunday
    const weekStart = new Date(date.setDate(diff));
    weekStart.setHours(0, 0, 0, 0);
    return weekStart;
  }
}

export { ArrayDateData, DateData }
