import React from "react";
import * as d3 from "d3";

interface IProps {
    name: string;
    url: string;
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

    constructor(arrayOfRawData: Array<RawCalendarData>) {
        this.data = arrayOfRawData['data'].map(d => new CalendarData(d.timestamp, d.value));
    }

    // get list of dates as string
    getDates() : Array<string>{
        return this.data.map(d => d.date.toISOString().split('T')[0]);
    }

    // get value for date
    getValue(date: Date) : number {
        return this.data[this.isDateInArrayIndex(date)].value;
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


class CalendarViz extends React.Component<IProps, IState> {
    ref!: SVGSVGElement;
    name: string = this.props.name;
    url: string = this.props.url + this.name;


    private buildCalendar(url: string, name: string) {
        // Set the dimensions of the calendar heatmap
        const width = 960;
        const height = 136;
        const cellSize = 17;

        // Set the colors for the calendar heatmap
        const color = d3
            .scaleQuantize<string>()
            .range([
                "#EBF7E3",
                "#9BD770",
                "#66B032",
                "#375F1B",
                "#1B3409",
            ]);

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
            .append('g')
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
            .text(function (d) {
                return d;
            });

        // Append the day labels to the calendar heatmap
        const rect = svg
            .append('g')
            .attr("fill", "none")
            .attr("stroke", "#ccc")
            .selectAll("rect")
            .data(function (d) {
                return d3.timeDays(new Date(d, 0, 1), new Date(d + 1, 0, 1));
            })
            .enter()
            .append("rect")
            .attr("width", cellSize)
            .attr("height", cellSize)
            .attr("x", function (d) {
                return d3.timeWeek.count(d3.timeYear(d), d) * cellSize;
            })
            .attr("y", function (d) {
                return d.getDay() * cellSize;
            })
            .datum(d3.timeFormat("%Y-%m-%d"));

        d3.json(url).then(data => {
            let d3data = Object.assign(new Array<RawCalendarData>(), data);
            let calendarData = new ArrayCalendarData(d3data);
            color.domain([0, 4]);
            rect.filter(function (d) {
                return calendarData.isDateInArray(new Date(d));
            }).attr('fill', function (d) {
                return color(calendarData.getValue(new Date(d)));
            })
        })

    }

    componentDidMount() {
        this.buildCalendar(this.url, this.name);
    }

    render() {
        return (
            <div className={this.name}>
                <h1>{this.name}</h1>
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

export default CalendarViz;