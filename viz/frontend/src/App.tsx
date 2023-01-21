    import React from "react";
    import "./App.scss";
    import CalendarViz from "./components/CalendarViz";

    interface IProps {}
    interface IState {}

    class App extends React.Component<IProps, IState> {

      baseUrl: string = "http://localhost:8080/api/data/";

      render() {
        return (
            <div className="App">
              <CalendarViz name="happyLevels"  url={this.baseUrl}/>
              <CalendarViz name="anxietyLevels"  url={this.baseUrl}/>
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
