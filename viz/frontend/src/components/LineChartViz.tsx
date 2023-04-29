import React , {useState} from "react";
import * as d3 from "d3";
import { Col } from "antd";
import styles from "../stylesheets.module.scss";
import { ArrayDateData, RawDateData } from "src/models/date_data";
import { getLastDateToBeShownInViz, getStartDateToBeShownInViz } from "src/utils/date";
import { viz_details } from "src/models/constants";
import Tooltip from './Tooltip';

interface IProps {
  name: string;
  displayName: string;
  maxRange: number;
  minRange: number;
  isPositive: boolean;
  url: string;
}

interface IState {
  tooltipData: {
    visible: boolean;
    content: string;
  };
  tooltipPosition: {
    left: number;
    top: number;
  };
}

class LineChartViz extends React.Component<IProps, IState> {
  ref!: SVGSVGElement;
  name: string = this.props.name;
  url: string = this.props.url + this.name;
  displayName: string = this.props.displayName;
  maxRange: number = this.props.maxRange;
  minRange: number = this.props.minRange;
  isPositive: boolean = this.props.isPositive;
  constructor(props: IProps) {
    super(props);

    this.state = {
      tooltipData: {
        visible: false,
        content: '',
      },
      tooltipPosition: {
        left: 0,
        top: 0,
      },
    };
  }

  private buildChart(url: string, name: string) {
    const margin = { top: 20, right: 20, bottom: 50, left: 50 };

    const width = viz_details.viz_width - margin.left - margin.right;
    const height = viz_details.viz_height - margin.top - margin.bottom;

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

    let lastDayForViz = getLastDateToBeShownInViz(new Date());
    let startDayForViz = getStartDateToBeShownInViz(new Date());
    var tooltip = d3.select("body")
    .append("div")
    .attr("id", "tooltip")
    .style("opacity", 0);

     x.domain([startDayForViz, lastDayForViz]);
      svg
        .append("g")
        .attr("class", "x axis")
        .style("stroke-width", viz_details.graph_line_width)
        .attr("transform", "translate(0," + height + ")")
        .call(d3.axisBottom(x))
        .selectAll("text") 
        .style("font-size", "8px") 
        // .attr("y", 40); 

      // eslint-disable-next-line eqeqeq
      if (this.maxRange == 0 && this.minRange == 0) {
        const maxVal = d3.max(chartData.getData(), (d) => Math.abs(d.value)) as number; 
        y.domain([0, maxVal]);
      } else {
        y.domain([this.minRange, this.maxRange]);
      }
      svg.append("g").attr("class", "y axis").call(d3.axisLeft(y))
        .style("stroke-width", viz_details.graph_line_width)

      svg
        .append("path")
        .datum(chartData.getData())
        .attr("class", "line")
        .attr("d", line)
        .style("fill", "none")
        .style("stroke", colour)
        .style("stroke-width", 1.7);

        svg
        .selectAll(".dot")
        .data(chartData.getData())
        .enter()
        .append("circle")
        .attr("class", "dot")
        .attr("cx", (d) => x(d.date))
        .attr("cy", (d) => y(Math.abs(d.value)))
        .attr("r", 3) // Adjust the radius of the dots as needed
        .style("fill",  colourDark) // Change the fill color of the dots as needed
        .on("mouseover", (event, d) => {
          this.setState({
            tooltipData: {
              visible: true,
              content: `Date: ${d.date}<br/>Value: ${d.value}`,
            },
            tooltipPosition: {
              left: event.clientX,
              top: event.clientY,
            },
          });
        })
        .on("mousemove", (event, d) => {
          this.setState({
            tooltipPosition: {
              left: event.clientX,
              top: event.clientY,
            },
          });
        })
        .on("mouseout", () => {
          this.setState({
            tooltipData: {
              visible: false,
              content: '',
            },
          });
        });
  

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
          <Tooltip tooltipData={this.state.tooltipData} position={this.state.tooltipPosition} />
        </div>
      </Col>
    );
  }
}

export default LineChartViz;
