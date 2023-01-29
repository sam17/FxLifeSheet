    import React from "react";
    import "./App.scss";
    import CalendarViz from "./components/CalendarViz";

    interface IProps {}
    interface IState {}

    class App extends React.Component<IProps, IState> {

      baseUrl: string = "/api/data/";

      render() {
        return (
            <div className="App">
              <CalendarViz isPositive={true} minRange={0} maxRange={5} name="happyLevels"  url={this.baseUrl}/>
              <CalendarViz isPositive={false} minRange={0} maxRange={5} name="anxietyLevels"  url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={5} name="energyLevels"  url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={5} name="learned"  url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={1} name="socializing"  url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={1} name="parents"  url={this.baseUrl}/>
              <CalendarViz isPositive={false} minRange={0} maxRange={5} name="lays"  url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={1} name="gym"  url={this.baseUrl}/>
              <CalendarViz isPositive={true} minRange={0} maxRange={1} name="numberOfMeals"  url={this.baseUrl}/>
            </div>
        );
      }
    }

    export default App;
