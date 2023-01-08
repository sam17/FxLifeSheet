import React from "react";
import * as d3 from "d3";
import "./App.scss";
import { createImportSpecifier } from "typescript";
import { treemapBinary } from "d3";

// Copyright 2021 Observable, Inc.
// Released under the ISC license.
// https://observablehq.com/@d3/calendar-view
// function Calendar(data: Array<CalendarData>, {
//   x = ([x]) => x, // given d in data, returns the (temporal) x-value
//   y = ([, y]) => y, // given d in data, returns the (quantitative) y-value
//   title = "random", // given d in data, returns the title text
//   width = 928, // width of the chart, in pixels
//   cellSize = 17, // width and height of an individual day, in pixels
//   weekday = "monday", // either: weekday, sunday, or monday
//   formatDay = i => "SMTWTFS"[i], // given a day number in [0, 6], the day-of-week label
//   formatMonth = "%b", // format specifier string for months (above the chart)
//   yFormat= "$", // format specifier string for values (in the title)
//   colors = d3.interpolatePiYG
// } = {}) {
//   // Compute values.
//   const X = d3.map(data.map(d => d.date), x);
//   const Y = d3.map(data.map(d = d.value), y);
//   const I = d3.range(X.length);

//   const countDay = weekday === "sunday" ? i => i : i => (i + 6) % 7;
//   const timeWeek = weekday === "sunday" ? d3.utcSunday : d3.utcMonday;
//   const weekDays = weekday === "weekday" ? 5 : 7;
//   const height = cellSize * (weekDays + 2);

//   // Compute a color scale. This assumes a diverging color scheme where the pivot
//   // is zero, and we want symmetric difference around zero.
//   const max = d3.quantile(Y, 0.9975, Math.abs);
//   const color = d3.scaleSequential([-max, +max], colors).unknown("none");

//   // Construct formats.
//   formatMonth = d3.utcFormat(formatMonth);

//   // Compute titles.
//   if (title === undefined) {
//     const formatDate = d3.utcFormat("%B %-d, %Y");
//     const formatValue = color.tickFormat(100, yFormat);
//     title = i => `${formatDate(X[i])}\n${formatValue(Y[i])}`;
//   } else if (title !== null) {
//     const T = d3.map(data, title);
//     title = i => T[i];
//   }

//   // Group the index by year, in reverse input order. (Assuming that the input is
//   // chronological, this will show years in reverse chronological order.)
//   const years = d3.groups(I, i => X[i].getUTCFullYear()).reverse();

//   function pathMonth(t) {
//     const d = Math.max(0, Math.min(weekDays, countDay(t.getUTCDay())));
//     const w = timeWeek.count(d3.utcYear(t), t);
//     return `${d === 0 ? `M${w * cellSize},0`
//         : d === weekDays ? `M${(w + 1) * cellSize},0`
//         : `M${(w + 1) * cellSize},0V${d * cellSize}H${w * cellSize}`}V${weekDays * cellSize}`;
//   }

//   const svg = d3.create("svg")
//       .attr("width", width)
//       .attr("height", height * years.length)
//       .attr("viewBox", [0, 0, width, height * years.length])
//       .attr("style", "max-width: 100%; height: auto; height: intrinsic;")
//       .attr("font-family", "sans-serif")
//       .attr("font-size", 10);

//   const year = svg.selectAll("g")
//     .data(years)
//     .join("g")
//       .attr("transform", (d, i) => `translate(40.5,${height * i + cellSize * 1.5})`);

//   year.append("text")
//       .attr("x", -5)
//       .attr("y", -5)
//       .attr("font-weight", "bold")
//       .attr("text-anchor", "end")
//       .text(([key]) => key);

//   year.append("g")
//       .attr("text-anchor", "end")
//     .selectAll("text")
//     .data(weekday === "weekday" ? d3.range(1, 6) : d3.range(7))
//     .join("text")
//       .attr("x", -5)
//       .attr("y", i => (countDay(i) + 0.5) * cellSize)
//       .attr("dy", "0.31em")
//       .text(formatDay);

//   const cell = year.append("g")
//     .selectAll("rect")
//     .data(weekday === "weekday"
//         ? ([, I]) => I.filter(i => ![0, 6].includes(X[i].getUTCDay()))
//         : ([, I]) => I)
//     .join("rect")
//       .attr("width", cellSize - 1)
//       .attr("height", cellSize - 1)
//       .attr("x", i => timeWeek.count(d3.utcYear(X[i]), X[i]) * cellSize + 0.5)
//       .attr("y", i => countDay(X[i].getUTCDay()) * cellSize + 0.5)
//       .attr("fill", i => color(Y[i]));

//   if (title) cell.append("title")
//       .text(title);

//   const month = year.append("g")
//     .selectAll("g")
//     .data(([, I]) => d3.utcMonths(d3.utcMonth(X[I[0]]), X[I[I.length - 1]]))
//     .join("g");

//   month.filter((d, i) => i).append("path")
//       .attr("fill", "none")
//       .attr("stroke", "#fff")
//       .attr("stroke-width", 3)
//       .attr("d", pathMonth);

//   month.append("text")
//       .attr("x", d => timeWeek.count(d3.utcYear(d), timeWeek.ceil(d)) * cellSize + 2)
//       .attr("y", -5)
//       .text(formatMonth);

//   return Object.assign(svg.node(), {scales: {color}});
// }

interface IProps {}

interface IState {}

interface CalendarData {
  date: Date;
  values: number;
}

class ArrayCalendarData {
  data: Array<CalendarData> = [];

  // constructor(data: Array<CalendarData>) {
  //   this.data = data;
  // } 

  get(date: Date) {
    return this.data.find(d => d.date.getTime() === date.getTime())?.values;
  }

  // get list of dates as string
  getDates() : Array<string>{
    return this.data.map(d => new Date(d.date).toISOString().split('T')[0]);
  }

  // get value for date
  getValue(date: Date) : number {
    return this.data[this.isDateInArrayIndex(date)].values;   
  }

  // compare two dates
  compareDates(a: Date, b: Date) : boolean {
    let x = new Date(a);
    let y = new Date(b);
    x.setHours(0,0,0);
    y.setHours(0,0,0);
    return x.getTime() === y.getTime();
  }

  isDateInArray(a: Date) {
    return this.data.some(d => this.compareDates(d.date, a));
  } 

  // above function but returns the index
  isDateInArrayIndex(a: Date) {
    return this.data.findIndex(d => this.compareDates(d.date, a));
  }


  public getData() : Array<CalendarData> {
    return this.data;
  }
}

class App extends React.Component<IProps, IState> {
  ref!: SVGSVGElement;

  private buildCalendar() {
    // Set the dimensions of the calendar heatmap
    const width = 960;
    const height = 136;
    const cellSize = 17;

    // Set the colors for the calendar heatmap
    const color = d3
      .scaleQuantize<string>()
      .range([
        "#ffffd9",
        "#edf8b1",
        "#c7e9b4",
        "#7fcdbb",
        "#41b6c4",
        "#1d91c0",
        "#225ea8",
        "#253494",
        "#081d58",
      ]);

    // Create the SVG element for the calendar heatmap
    const svg = d3
      .select("body")
      .selectAll("svg")
      .data(d3.range(2017, 2020))
      .enter()
      .append("svg")
      .attr("width", width)
      .attr("height", height)
      .attr("class", "RdYlGn")
      .append("g")
      .attr(
        "transform",
        "translate(" +
          (width - cellSize * 53) / 2 +
          "," +
          (height - cellSize * 7 - 1) +
          ")"
      );

    // Append the month labels to the calendar heatmap
    svg
      .append("text")
      .attr("transform", "translate(-6," + cellSize * 3.5 + ")rotate(-90)")
      .style("text-anchor", "middle")
      .text(function(d) {
        return d;
      });

    // Append the day labels to the calendar heatmap
    const rect = svg
      .append("g")
      .attr("fill", "none")
      .attr("stroke", "#ccc")
      .selectAll("rect")
      .data(function(d) {
        return d3.timeDays(new Date(d, 0, 1), new Date(d + 1, 0, 1));
      })
      .enter()
      .append("rect")
      .attr("width", cellSize)
      .attr("height", cellSize)
      .attr("x", function(d) {
        return d3.timeWeek.count(d3.timeYear(d), d) * cellSize;
      })
      .attr("y", function(d) {
        return d.getDay() * cellSize;
      })
      .datum(d3.timeFormat("%Y-%m-%d"));

    d3.json("https://gist.githubusercontent.com/sam17/cb0c4a4134e169c86f76beecb91e4aa7/raw/95b93d68460df68a4d07e62a6c3519d5399e76bc/data.json").then(data => {

      let d3data = Object.assign(new ArrayCalendarData(), data);

      color.domain([0, 4]);

       console.log(typeof d3data.getDates() )
    
        rect.filter(function (d) { 
          return d3data.isDateInArray(new Date(d));
        })
        .attr('fill', function (d) { return color(d3data.getValue(new Date(d))); })
      console.log(data);
    })
 
  }

  componentDidMount() {
    // activate
    // this.buildGraph([5, 10, 12]);
    this.buildCalendar();
  }

  render() {
    return (
      <div className="svg">
        <svg
          className="container"
          ref={(ref: SVGSVGElement) => (this.ref = ref)}
          width="100"
          height="100"
        ></svg>
      </div>
    );
  }
}

export default App;
