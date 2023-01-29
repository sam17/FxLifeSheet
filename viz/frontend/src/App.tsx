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
