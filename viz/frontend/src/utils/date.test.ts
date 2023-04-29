import { getLastDateToBeShownInViz, getStartDateToBeShownInViz } from './date';

describe('getLatestDate', () => {
    it('should return the next Saturday after the current date', () => {
        const scenarios = [
          { currentDate: '2023-05-01', expectedLatestDate: '2023-05-06' }, // Sunday
          { currentDate: '2023-04-29', expectedLatestDate: '2023-05-06' }, // Saturday
          { currentDate: '2023-05-02', expectedLatestDate: '2023-05-06' }, // Monday
          { currentDate: '2023-05-03', expectedLatestDate: '2023-05-06' }, // Tuesday
          { currentDate: '2023-05-04', expectedLatestDate: '2023-05-06' }, // Wednesday
          { currentDate: '2023-05-05', expectedLatestDate: '2023-05-06' }, // Thursday
          { currentDate: '2023-05-06', expectedLatestDate: '2023-05-13' }  // Friday
        ];
    
        scenarios.forEach(({ currentDate, expectedLatestDate }) => {
            var dateString = formatDateFromISOString(getLastDateToBeShownInViz(new Date(currentDate)).toISOString());
            expect(dateString).toBe(expectedLatestDate);
        });
      });

        it('should return the x weeks before', () => {
          const scenarios = [
          { currentDate: '2023-05-01', expectedLatestDate: '2023-01-07' }, // Sunday
        //   { currentDate: '2023-04-29', expectedLatestDate: '2023-04-08' }, // Saturday
        //   { currentDate: '2023-04-05', expectedLatestDate: '2023-03-11' }, // Monday
        //   { currentDate: '2023-05-03', expectedLatestDate: '2023-05-06' }, // Tuesday
        //   { currentDate: '2023-05-04', expectedLatestDate: '2023-05-06' }, // Wednesday
        //   { currentDate: '2023-05-05', expectedLatestDate: '2023-05-06' }, // Thursday
        //   { currentDate: '2023-05-06', expectedLatestDate: '2023-05-13' }  // Friday
            ];
            scenarios.forEach(({ currentDate, expectedLatestDate }) => {
                var dateString = formatDateFromISOString(getStartDateToBeShownInViz(new Date(currentDate)).toISOString());
                expect(dateString).toBe(expectedLatestDate);
            });
        });
});

function formatDateFromISOString(isoString) {
    const [year, month, day] = isoString.split('T')[0].split('-');
    return `${year}-${month}-${day}`;
  }