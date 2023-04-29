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
}

function DateElement() {
    return (
        <div className="date-element">
            {getStringForDate()}
        </div>
    );
};
export default DateElement;