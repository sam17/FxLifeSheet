    import React from "react";
    import "./App.scss";
    import { Row, Col } from 'antd';
    import CalendarViz from "./components/CalendarViz";

    interface IProps {}
    interface IState {
        name: string;
    }

    class App extends React.Component<IProps, IState> {
      constructor(props: IProps) {
          super(props);
          this.state = {
              name: "unnamed"
          }
      }

      development: boolean = !process.env.NODE_ENV || process.env.NODE_ENV === 'development';
      baseUrl : string = this.development?"https://metrics.soumyadeep.in/api/data/": "/api/data/";

      getMetadata = () => {
        return fetch(this.baseUrl + "metadata").then(response => response.json());
      }

      componentDidMount() {
          this.getMetadata().then(data => {
              console.log(data);
              this.setState({name: data.name});
          }).catch(error => {
              console.log(error);
              this.setState({name: "Error"});
          })
      }

      render() {
        return (
            <div className="App">
                <h1><center>How is {this.state.name}?</center></h1>
                <Col>
                    <CalendarViz name="happyLevels"  url={this.baseUrl}/>
                    <CalendarViz name="anxietyLevels"  url={this.baseUrl}/>
                </Col>
                <CalendarViz name="energyLevels"  url={this.baseUrl}/>
                <CalendarViz name="learned"  url={this.baseUrl}/>
                <CalendarViz name="socializing"  url={this.baseUrl}/>
                <CalendarViz name="parents"  url={this.baseUrl}/>
                <CalendarViz name="lays"  url={this.baseUrl}/>
                <CalendarViz name="gym"  url={this.baseUrl}/>
                <CalendarViz name="numberOfMeals"  url={this.baseUrl}/>
            </div>
        );
      }
    }

    export default App;
