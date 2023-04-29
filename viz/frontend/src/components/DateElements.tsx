import React from 'react';
import { getStartDateToBeShownInViz, getLastDateToBeShownInViz } from 'src/utils/date';


function getStringForDate() {
    var startString = getDateInString(getStartDateToBeShownInViz(new Date()));
    var endString = getDateInString(getLastDateToBeShownInViz(new Date()));
    return startString + " - " + endString;
}

function getDateInString(date: Date) {
    var dateString = date.toDateString();    
    return dateString.substring(4, dateString.length);
    // const isoString = date.toISOString();
    // const [year, month, day] = isoString.split('T')[0].split('-');
    // return `${year}-${month}-${day}`;
}

function DateElement() {
    return (
        <div className="date-element">
            {getStringForDate()}
        </div>
    );
};
export default DateElement;