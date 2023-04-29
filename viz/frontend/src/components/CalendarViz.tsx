import React from "react";
import * as d3 from "d3";
import { Col } from "antd";
import styles from "../stylesheets.module.scss";
import { ArrayDateData, RawDateData } from "src/models/date_data";
import { getLastDateToBeShownInViz, getStartDateToBeShownInViz, weeksToShowInViz } from "src/utils/date";

interface IProps {
  name: string;
  displayName: string;
  url: string;
  maxRange: number;
  minRange: number;
  isPositive: boolean;
  isReverse: boolean;
  cadence: string;
}

interface IState {}

class CalendarViz extends React.Component<IProps, IState> {
  ref!: SVGSVGElement;
  name: string = this.props.name;
  url: string = this.props.url + this.name;
  maxRange: number = this.props.maxRange;
  minRange: number = this.props.minRange;
  isPositive: boolean = this.props.isPositive;
  isReverse: boolean = this.props.isReverse;
  displayName: string = this.props.displayName;
  cadence: string = this.props.cadence;

  private buildCalendar(url: string, name: string, cadence: string) {
    // Set the dimensions of the calendar heatmap
    const width = 960;
    const height = 126;
    const cellSize = 17;

    // Set the colors for the calendar heatmap
    const positiveColors = d3
      .scaleQuantize<string>()
      .range(["#EBF7E3", "#9BD770", "#66B032", "#375F1B", "#1B3409"]);

    const negativeColors = d3
      .scaleQuantize<string>()
      .range(["#F7E3E3", "#D77070", "#B03232", "#5F1B1B", "#340909"]);

    
    const startDayForViz = getStartDateToBeShownInViz(new Date())
    const lastDayForViz = getLastDateToBeShownInViz(new Date())

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

    const rect = svg
        .append("g")
        .attr("fill", "none")
        .attr("stroke", "#ccc")
        .selectAll("rect")
        .data(function(d) {
          if (cadence === "week") {
            return d3.timeWeeks(new Date(d, 0, 1), new Date(d + 1, 0, 1));
          } else {
            return d3.timeDays(new Date(d, 0, 1), new Date(d + 1, 0, 1));
          }
        })
        .enter()
        .append("rect")
        .filter(function(d) {
          if ( d > lastDayForViz || d < startDayForViz ){
            return false;
          }
          return true;
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

    let y_offset = cellSize * 3;
    if (cadence === "day") {
      y_offset = y_offset + (cellSize * 7);
    }

    d3.json(url).then((data) => {
      let d3data = Object.assign(new Array<RawDateData>(), data);
      let calendarData = new ArrayDateData(d3data['data'], this.props.maxRange, this.props.minRange, this.props.isPositive, this.props.isReverse);

      let color = this.props.isPositive ? positiveColors : negativeColors;
      color.domain([this.props.minRange, this.props.maxRange]);

      rect
          .filter(function (d) {
            if (cadence === "week") {
              return calendarData.getValueInWeekOfDate(new Date(d)).date !== null;
            } else {
              return calendarData.dateExistsInArray(new Date(d));
            }
          })
          .attr("fill", function (d) {
            if (cadence === "week") {
              const value = calendarData.getValueInWeekOfDate(new Date(d)).value;
              return value !== null ? color(value) : "gray"; // Replace "gray" with your default color
            } else {
              return color(calendarData.getModifiedValue(new Date(d)));
            }
          });

      //TODO(dementor): This will break when year changes but future soumyadeep can fix it
      const year = calendarData['data'][0].date.getFullYear();

      svg.append("text")
          .attr("class", "yearLabel")
          .attr("x", cellSize*20 / 2)
          .attr("y", y_offset + cellSize)
          .style("text-anchor", "middle")
          .text(year);
    });

    // Append the week labels to the calendar heatmap
    const weekLabels = svg.selectAll(".weekLabel")
        .data(d3.range(1, weeksToShowInViz + 1))
        .enter().append("text")
        .text(function(d) { return "W" + d; })
        .attr("x", function(d) { return (d - 1) * cellSize + (cellSize / 2); })
        .attr("y", y_offset )
        .style("text-anchor", "middle")
        .attr("class", "weekLabel")
        .attr("font-size", "6px");

  }

  componentDidMount() {
    this.buildCalendar(this.url, this.name, this.cadence);
  }

  render() {
    return (
      <Col xxl={6} xl={8} lg={8} md={12} sm={24} xs={24} >
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
