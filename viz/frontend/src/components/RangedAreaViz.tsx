import React from "react";
import c3 from "c3";
import 'c3/c3.css';
import {DMYData} from "../models/DatedData";

interface IProps {
    displayName: string;
    url: string;
    rangeKey: string;
    targetKey: string;
}

class RangedData {
    date: string;
    lower?: number;
    upper?: number;
    actual?: number;

    constructor(timestamp: string) {
        this.date =  new Date(timestamp).toISOString().split("T")[0];
    }

    setActual(actual: number) {
        this.actual = actual;
    }
}

interface IState {}

class RangedAreaViz extends React.Component<IProps, IState> {

    private getActual = () => {
        let rangedData: RangedData[] = [];

        return fetch(this.props.url + this.props.targetKey).then((response) => response.json())
            .then((data) => {
                data['data'].map(
                    (d) => {
                        let rangeData = new RangedData(d.timestamp);
                        rangeData.setActual(d.value);
                        rangedData.push(rangeData);
                    }
                );
                return rangedData;
            });
    };

    private getRange = () => {
        let rangedData: RangedData[] = [];
        return fetch(this.props.url + this.props.rangeKey).then((response) => response.json())
            .then((data) => {
                data['data'].map(
                    (d) => {
                        let upper = d.value != 0? ((d.value * 4) - 1): 0;
                        let lower = d.value != 0?((d.value - 1) * 4): 0;
                        let rangeData = new RangedData(d.timestamp);
                        rangeData.upper = upper;
                        rangeData.lower = lower;
                        rangedData.push(rangeData);
                    }
                );
                return rangedData;
            });
    }


    private buildRangedAreaViz(rangeData: RangedData[]) {
        c3.generate({
            bindto: "#rangedChart",
            data: {
                x: 'x',
                columns: [
                    ['x'].concat(rangeData.map((d) => d.date)),
                    ['Actual'].concat(rangeData.map((d) => d.actual!.toString())),
                    ['upperBound'].concat(rangeData.map((d) => d.upper!.toString())),
                    ['lowerBound'].concat(rangeData.map((d) => d.lower!.toString())),
                ]
            },
            types: {
                data1: 'area',
                data2: 'line',
                data3: 'area'
            },
            colors: {
                data1: 'hsl(90, 70%, 90%)',
                data2: 'hsl(120, 100%, 60%)',
                data3: 'white'
            },
            axis: {
                x: {
                    type: 'timeseries',
                    tick: {
                        format: '%Y-%m-%d'
                    }
                }
            },
            point: {
                r: 3
            },
        });
//         c3.generate({
//             bindto: "#rangedChart",
//
//             data: {
//                 colors: {
//                     data1: 'hsl(90, 70%, 90%)',
//                     data2: 'hsl(120, 100%, 60%)',
//                     data3: 'white'
//                 },
//                 columns: [
//                     ['data1', 300, 350, 300, 290, 225, 220],
//                     ['data2', 250, 320, 280, 250, 170, 180],
//                     ['data3', 230, 300, 240, 200, 150, 150]
//                 ],
//                 types: {
//                     data1: 'area',
//                     data2: 'line',
//                     data3: 'area'
//                 },
//                 point: {
//                     r: 7
//                 },
//             },
//         });
    }

    private checkConsecutiveDate(dateA: string, dateB: string): boolean {
        let dateAObj = new Date(dateA);
        let dateBObj = new Date(dateB);
        return dateAObj.getDate() === dateBObj.getDate() - 1;
    }

    componentDidMount() {
        Promise.all([this.getActual(), this.getRange()]).then((data) => {
            console.log("promise all", data);
            let actualData = data[0];
            let rangeData = data[1];
            let mergedRangeData: RangedData[] = [];
            actualData.map((actual) => {
                rangeData.map((range) => {
                    // console.log("actual", actual.date, "range", range.date);
                    if (this.checkConsecutiveDate(range.date, actual.date)) {
                        // console.log("match");
                        let rangeData = new RangedData(actual.date);
                        rangeData.actual = actual.actual;
                        rangeData.upper = range.upper;
                        rangeData.lower = range.lower;
                        mergedRangeData.push(rangeData);
                    }
                });
            });
            console.log("mergedRangeData", mergedRangeData);
            this.buildRangedAreaViz(mergedRangeData);
        });
    }

    render() {
        return (
            <div id="rangedChart"/>
        );
    }

}

export default RangedAreaViz;
