import React from "react";
import c3 from "c3";
import 'c3/c3.css';

interface IProps {}

interface IState {}

class RangedAreaViz extends React.Component<IProps, IState> {

    private buildRangedAreaViz() {
        c3.generate({
            bindto: "#rangedChart",

            data: {
                colors: {
                    data1: 'hsl(90, 70%, 90%)',
                    data2: 'hsl(120, 100%, 60%)',
                    data3: 'white'
                },
                columns: [
                    ['data1', 300, 350, 300, 290, 225, 220],
                    ['data2', 250, 320, 280, 250, 170, 180],
                    ['data3', 230, 300, 240, 200, 150, 150]
                ],
                types: {
                    data1: 'area',
                    data2: 'line',
                    data3: 'area'
                },
                point: {
                    r: 7
                },
            },
        });
    }

    componentDidMount() {
        this.buildRangedAreaViz();
    }

    render() {
        return (
            <div id="rangedChart"/>
        );
    }

}

export default RangedAreaViz;
