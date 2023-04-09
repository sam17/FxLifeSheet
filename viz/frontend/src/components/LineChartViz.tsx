import React from "react";
import * as d3 from "d3";
import { Col } from "antd";
import styles from "../stylesheets.module.scss";
import { ArrayDateData, RawDateData } from "src/models/date_data";

interface IProps {
  name: string;
  displayName: string;
  maxRange: number;
  minRange: number;
  isPositive: boolean;
  url: string;
}

interface IState {}

class LineChartViz extends React.Component<IProps, IState> {
  ref!: SVGSVGElement;
  name: string = this.props.name;
  url: string = this.props.url + this.name;
  displayName: string = this.props.displayName;
  maxRange: number = this.props.maxRange;
  minRange: number = this.props.minRange;
  isPositive: boolean = this.props.isPositive;

  private buildChart(url: string, name: string) {
    const margin = { top: 20, right: 20, bottom: 50, left: 50 };
    const width = 500 - margin.left - margin.right;
    const height = 200 - margin.top - margin.bottom;

    const x = d3.scaleTime().range([0, width]);
    const y = d3.scaleLinear().range([height, 0]);

    const positiveColor = "#375F1B";
    const negativeColor = "#5F1B1B";

    const colour = this.isPositive ? positiveColor : negativeColor;

    const positiveColorDark = "#1B3409";
    const negativeColorDark = "#340909";

    const colourDark = this.isPositive ? positiveColorDark : negativeColorDark;

    const line = d3
      .line<{ date: Date; value: number }>()
      .x((d) => x(d.date))
      .y((d) => y(Math.abs(d.value)));

    const svg = d3
      .select("." + this.name + "12")
      .selectAll("svg")
      .data(d3.range(2022, 2024))
      .enter()
      .append("svg")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .append("g")
      .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    d3.json(url).then((data) => {
      let d3data = Object.assign(new Array<RawDateData>(), data);
      let chartData = new ArrayDateData(
        d3data['data'],
        0,
        0,
        false,
        false
        // this.aggregation
      );

      x.domain(
        d3.extent(chartData.getData(), (d) => new Date(d.date)) as [Date, Date]
      );

    let today = new Date();

     x.domain([new Date("2023-01-01"), today]);
      svg
        .append("g")
        .attr("class", "x axis")
        .attr("transform", "translate(0," + height + ")")
        .call(d3.axisBottom(x))
        .selectAll("text") 
        .style("font-size", "8px") 
        // .attr("y", 40); 


      // eslint-disable-next-line eqeqeq
      if (this.maxRange == 0 && this.minRange == 0) {
        y.domain(
          d3.extent(chartData.getData(), (d) => Math.abs(d.value)) as [
            number,
            number
          ]
        );
      } else {
        y.domain([this.minRange, this.maxRange]);
      }
      svg.append("g").attr("class", "y axis").call(d3.axisLeft(y));

      svg
        .append("path")
        .datum(chartData.getData())
        .attr("class", "line")
        .attr("d", line)
        .style("fill", "none")
        .style("stroke", colour)
        .style("stroke-width", 0.75);

        svg
        .selectAll(".dot")
        .data(chartData.getData())
        .enter()
        .append("circle")
        .attr("class", "dot")
        .attr("cx", (d) => x(d.date))
        .attr("cy", (d) => y(Math.abs(d.value)))
        .attr("r", 1) // Adjust the radius of the dots as needed
        .style("fill",  colourDark); // Change the fill color of the dots as needed

    });
  }

  componentDidMount() {
    this.buildChart(this.url, this.name);
  }

  render() {
    return (
      <Col xxl={6} xl={8} lg={8} md={12} sm={24} xs={24}>
        <div className={this.name + "12"}>
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

export default LineChartViz;
