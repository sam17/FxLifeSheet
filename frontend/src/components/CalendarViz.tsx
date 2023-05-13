import React from "react";
import * as d3 from "d3";
import { Col } from "antd";
import styles from "../stylesheets.module.scss";
import { ArrayDateData, RawDateData } from "src/models/date_data";
import { getDateInString, getLastDateToBeShownInViz, getStartDateToBeShownInViz, weeksToShowInViz } from "src/utils/date";
import {viz_details} from "../models/constants";
import { tooltipData } from "./Tooltip";

interface IProps {
  name: string;
  displayName: string;
  url: string;
  maxRange: number;
  minRange: number;
  isPositive: boolean;
  isReverse: boolean;
  cadence: string;
  setTooltipData: (tooltipData: tooltipData) => void;
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
    const margin = { top: 20, right: 0, bottom: 50, left: 20 };

    const cellSize = 17;
    // Set the dimensions of the calendar heatmap
    const width = viz_details.viz_width - margin.left - margin.right;
    let num_days = cadence === "week" ? 2 : 8;
    const height = (cellSize * num_days) + margin.top + margin.bottom;

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
      .attr("transform", "translate(" + margin.left + "," + margin.top + ")");


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
        
    let y_offset = cellSize * num_days;

    d3.json(url).then((data) => {
      let d3data = Object.assign(new Array<RawDateData>(), data);
      let calendarData = new ArrayDateData(d3data['data'], this.maxRange, this.minRange, this.isPositive, this.isReverse);

      let color = this.isPositive ? positiveColors : negativeColors;
      color.domain([this.minRange, this.maxRange]);

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
          })
          .on("mouseover", (event, d) => {
            // Set the tooltip data and make it visible
            const value = cadence === "week" ? calendarData.getValueInWeekOfDate(new Date(d)).value : calendarData.getModifiedValue(new Date(d));
            this.props.setTooltipData({
              visible: true,
              date: new Date(d),
              value: value?.toString() ?? "",
              isPositive: this.isPositive,
            });
          })
          .on("mouseout", () => {
            // Hide the tooltip
            this.props.setTooltipData({
              visible: false,
              date: new Date(),
              value: "",
              isPositive: this.isPositive,
            });
          });

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

    const dayFormat = d3.timeFormat("%a");
    const dayIndices = [0, 1, 2, 3, 4, 5, 6];

    if (cadence === "day") {
      svg.selectAll(".dayLabel")
          .data(dayIndices)
          .enter().append("text")
          .text(function (d) {
            return dayFormat(new Date(2023, 0, d)).charAt(0);
          }) // Get the first character of the day abbreviation
          .attr("x", 0)
          .attr("y", function (d, i) {
            return i * cellSize;
          })
          // .style("text-anchor", "end")
          .attr("transform", "translate(-10," + cellSize / 1.5 + ")")
          .style("font-weight", "300")
          .style("font-size", "8px");
    }

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
