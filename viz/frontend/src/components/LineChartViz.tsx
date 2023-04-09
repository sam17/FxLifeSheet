import React from "react";
import * as d3 from "d3";
import { Col } from "antd";
import styles from "../stylesheets.module.scss";

interface IProps {
  name: string;
  displayName: string;
  url: string;
  aggregation: "addition" | "minimum" | "maximum" | "average";
}

interface IState {}

class LineChartViz extends React.Component<IProps, IState> {
  ref!: SVGSVGElement;
  name: string = this.props.name;
  url: string = this.props.url + this.name;
  displayName: string = this.props.displayName;
  aggregation: "addition" | "minimum" | "maximum" | "average" = this.props.aggregation;

  private buildLineChart(url: string, name: string, aggregation: string) {
    const margin = { top: 20, right: 20, bottom: 30, left: 50 };
    const width = 960 - margin.left - margin.right;
    const height = 500 - margin.top - margin.bottom;

    const parseTime = d3.timeParse("%Y-%m-%d");

    const x = d3.scaleTime().range([0, width]);
    const y = d3.scaleLinear().range([height, 0]);

    const line = d3
      .line<{ date: Date; value: number }>()
      .x((d) => x(d.date))
      .y((d) => y(d.value));

    const svg = d3
      .select("." + this.name)
      .append("svg")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .append("g")
      .attr("transform", `translate(${margin.left},${margin.top})`);

    d3.json(url).then((data: Array<RawCalendarData>) => {
      const calendarData = new ArrayCalendarData(data, 0, 0, true, false);
      let aggregatedData: { [key: string]: number } = {};

      calendarData.getData().forEach((d) => {
        const dateStr = d.date.toISOString().split("T")[0];
        if (aggregatedData[dateStr] === undefined) {
          aggregatedData[dateStr] = d.value;
        } else {
          switch (aggregation) {
            case "addition":
              aggregatedData[dateStr] += d.value;
              break;
            case "minimum":
              aggregatedData[dateStr] = Math.min(aggregatedData[dateStr], d.value);
              break;
            case "maximum":
              aggregatedData[dateStr] = Math.max(aggregatedData[dateStr], d.value);
              break;
            case "average":
              aggregatedData[dateStr] =
                (aggregatedData[dateStr] + d.value) / 2;
              break;
            default:
              break;
          }
        }
      });

      const processedData = Object.entries(aggregatedData).map(([key, value]) => ({
        date: parseTime(key),
        value,
      }));

      x.domain(d3.extent(processedData, (d) => d.date));
      y.domain(d3.extent(processedData, (d) => d.value));

       // Add the x-axis
  svg
  .append("g")
  .attr("transform", `translate(0,${height})`)
  .call(d3.axisBottom(x));

// Add the y-axis
svg.append("g").call(d3.axisLeft(y));

// Add the line
svg
  .append("path")
  .data([processedData])
  .attr("class", "line")
  .attr("d", line);

// Add the title
svg
  .append("text")
  .attr("x", width / 2)
  .attr("y", 0 - margin.top / 2)
  .attr("text-anchor", "middle")
  .style("font-size", "16px")
  .style("font-weight", "bold")
  .text(this.displayName);
});

}

componentDidMount() {
this.buildLineChart(this.url, this.name, this.aggregation);
}

render() {
return (
<Col xxl={12} xl={12} lg={12} md={12} sm={24} xs={24}>
<div className={this.name}>
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

export default LineChartViz;