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
              <CalendarViz isPositive={true} minRange={0} maxRange={5} name="happyLevels" displayName={"Happy"}  url={this.baseUrl}/>
              <CalendarViz isPositive={false} minRange={0} maxRange={5} name="anxietyLevels" displayName={"Anxiety"}  url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={5} name="energyLevels"  displayName={"Energy"} url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={5} name="learned"  displayName={"Learned"} url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={1} name="socializing" displayName={"Socializing"}  url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={1} name="parents"  displayName={"Parents"} url={this.baseUrl}/>
              <CalendarViz isPositive={false} minRange={0} maxRange={5} name="lays"  displayName={"Lays"} url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={1} name="gym"  displayName={"Gym"} url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={1} name="numberOfMeals"  displayName={"Meals"} url={this.baseUrl}/>
            </div>
        );
      }
    }

    export default App;
