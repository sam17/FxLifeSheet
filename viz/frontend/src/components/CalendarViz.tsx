import React from "react";
import * as d3 from "d3";
import { Col } from "antd";
import styles from "../stylesheets.module.scss";

interface IProps {
  name: string;
  displayName: string;
  url: string;
  maxRange: number;
  minRange: number;
  isPositive: boolean;
  isReverse: boolean;
}

interface IState {}

interface RawCalendarData {
  timestamp: string;
  value: number;
}

class CalendarData {
  date: Date;
  value: number;

  constructor(timestamp: string, value: number) {
    this.date = new Date(timestamp);
    this.value = value;
  }
}

class ArrayCalendarData {
  data: Array<CalendarData> = [];
  maxRange: number = 0;
  minRange: number = 0; 
  isPositive: boolean = true;
  isReverse: boolean = false;

  constructor(arrayOfRawData: Array<RawCalendarData>, maxRange: number, minRange: number, isPositive: boolean, isReverse: boolean) {
    this.data = arrayOfRawData["data"].map(
      (d) => new CalendarData(d.timestamp, d.value)
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

  public getData(): Array<CalendarData> {
    return this.data;
  }
}

class CalendarViz extends React.Component<IProps, IState> {
  ref!: SVGSVGElement;
  name: string = this.props.name;
  url: string = this.props.url + this.name;
  maxRange: number = this.props.maxRange;
  minRange: number = this.props.minRange;
  isPositive: boolean = this.props.isPositive;
  isReverse: boolean = this.props.isReverse;
  displayName: string = this.props.displayName;

  private buildCalendar(url: string, name: string) {
    // Set the dimensions of the calendar heatmap
    const width = 960;
    const height = 136;
    const cellSize = 17;

    // Set the colors for the calendar heatmap
    const positiveColors = d3
      .scaleQuantize<string>()
      .range(["#EBF7E3", "#9BD770", "#66B032", "#375F1B", "#1B3409"]);

    const negativeColors = d3
      .scaleQuantize<string>()
      .range(["#F7E3E3", "#D77070", "#B03232", "#5F1B1B", "#340909"]);

    // Create the SVG element for the calendar heatmap
    const svg = d3
      .select("." + this.name)
      .selectAll("svg")
      .data(d3.range(2022, 2024))
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
      .filter(function(d) {
        if (d.getMonth() === 3 && d.getDate() === 30) {
          return false;
        }
        return d.getMonth() <= 3 ;
        })
      .attr("width", cellSize)
      .attr("height", cellSize)
      .attr("x", function(d) {
        return d3.timeWeek.count(d3.timeYear(d), d) * cellSize;
      })
      .attr("y", function(d) {
        return d.getDay() * cellSize;
      })
      .datum(d3.timeFormat("%Y-%m-%d"));


    d3.json(url).then((data) => {
      let d3data = Object.assign(new Array<RawCalendarData>(), data);
      let calendarData = new ArrayCalendarData(d3data, this.props.maxRange, this.props.minRange, this.props.isPositive, this.props.isReverse);

      let color = this.props.isPositive ? positiveColors : negativeColors;
      color.domain([this.props.minRange, this.props.maxRange]);
      rect
        .filter(function(d) {
          return calendarData.isDateInArray(new Date(d));
        })
        .attr("fill", function(d) {
          return color(calendarData.getValueModified(new Date(d)));
        });
    });
  }

  componentDidMount() {
    this.buildCalendar(this.url, this.name);
  }

  render() {
    return (
      <Col xs={24} sm={6}  >
      <div className={this.name}>
        <h2 className={styles.vizHeading}>{this.displayName}</h2>
        <svg
          className="container"
          ref={(ref: SVGSVGElement) => (this.ref = ref)}
          width="0"
          height="0"
        ></svg>
      </div>
    </Col>
    );
  }
}

export default CalendarViz;
