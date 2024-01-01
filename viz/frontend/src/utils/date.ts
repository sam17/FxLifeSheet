const weeksToShowInViz = 18;

function getLastDateToBeShownInViz(date: Date) {
  const daysUntilSaturday = 6 - date.getDay();

  if (daysUntilSaturday === 0) {
    date.setDate(date.getDate() + 7);
    return date;
  }

  date.setDate(date.getDate() + daysUntilSaturday);
  return date;
}

function getStartDateToBeShownInViz(date: Date) {
    var lastDate = getLastDateToBeShownInViz(date);
    date.setDate(lastDate.getDate() - 7 * weeksToShowInViz);
    return date;
}

function getDateInString(date: Date) {
    var dateString = date.toDateString();    
    return dateString.substring(4, dateString.length);
}

function getContinuousDates() {
  const startYear = 2022;
  const endYear = new Date().getFullYear(); // Or any end year you prefer
  const startDate = new Date(startYear, 0, 1);
  const endDate = new Date(endYear + 1, 0, 1); // Till the beginning of next year
  return [startDate, endDate];
}

export { getLastDateToBeShownInViz, getStartDateToBeShownInViz, weeksToShowInViz, getDateInString, getContinuousDates };
