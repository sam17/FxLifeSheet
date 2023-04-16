import { ArrayDateData, RawDateData } from "./date_data";

describe('ArrayDateData', () => {
    const rawDateData: RawDateData[] = [
        {timestamp: '2023-04-01T00:00:00.000Z', value: 10},
        {timestamp: '2023-04-02T00:00:00.000Z', value: 20},
        {timestamp: '2023-04-09T00:00:00.000Z', value: 30},
        {timestamp: '2023-03-27T00:00:00.000Z', value: 30},
        {timestamp: '2023-03-20T00:00:00.000Z', value: 20},
    ];

    const arrayDateData = new ArrayDateData(rawDateData, 100, 0, true, false);

    test('getDates', () => {
        const dates = arrayDateData.getDates();
        expect(dates).toEqual(['2023-04-01', '2023-04-02', '2023-04-09', '2023-03-27', '2023-03-20']);
    });

    test('getValue', () => {
        const value = arrayDateData.getValue(new Date('2023-04-02T00:00:00.000Z'));
        expect(value).toEqual(20);
    });

    test('getModifiedValue', () => {
        const modifiedValue = arrayDateData.getModifiedValue(new Date('2023-04-02T00:00:00.000Z'));
        expect(modifiedValue).toEqual(20);
    });

    test('areDatesEqual', () => {
        const isEqual = arrayDateData.areDatesEqual(new Date('2023-04-02T00:00:00.000Z'), new Date('2023-04-02T00:00:00.000Z'));
        expect(isEqual).toBeTruthy();
    });

    test('dateExistsInArray', () => {
        const exists = arrayDateData.dateExistsInArray(new Date('2023-04-02T00:00:00.000Z'));
        expect(exists).toBeTruthy();
    });

    test('findDateIndex', () => {
        const index = arrayDateData.findDateIndex(new Date('2023-04-02T00:00:00.000Z'));
        expect(index).toEqual(1);
    });

    test('getAverageValueForDate', () => {
        const average = arrayDateData.getAverageValueForDate(new Date('2023-04-02T00:00:00.000Z'));
        expect(average).toEqual(20);
    });


    test('getModifiedDataArray', () => {
        const modifiedDataArray = arrayDateData.getModifiedDataArray();
        expect(modifiedDataArray).toEqual([
            {date: new Date('2023-04-01T00:00:00.000Z'), value: 10},
            {date: new Date('2023-04-02T00:00:00.000Z'), value: 20},
            {date: new Date('2023-04-09T00:00:00.000Z'), value: 30},
            {date: new Date('2023-03-27T00:00:00.000Z'), value: 30},
            {date: new Date('2023-03-20T00:00:00.000Z'), value: 20},
        ]);
    });

    test('getData', () => {
        const data = arrayDateData.getData();
        expect(data.length).toEqual(5);
        expect(data[0].date).toEqual(new Date('2023-04-01T00:00:00.000Z'));
        expect(data[0].value).toEqual(10);
    });
});
