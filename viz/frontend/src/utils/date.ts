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

export { getLastDateToBeShownInViz, getStartDateToBeShownInViz, weeksToShowInViz };
