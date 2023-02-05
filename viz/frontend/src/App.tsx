import React from "react";
import "./App.scss";
import { Row, Col, Divider } from "antd";
import CalendarViz from "./components/CalendarViz";
import styles from "./stylesheets.module.scss";

interface IProps {}
interface IState {
  name: string;
}

class App extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      name: "unnamed",
    };
  }

  development: boolean = !process.env.NODE_ENV || process.env.NODE_ENV === "development";
  baseUrl: string = this.development ? "https://metrics.soumyadeep.in/api/" : "/api/data/";
  dataUrl = this.baseUrl + "data/";

  getMetadata = () => {
    return fetch(this.baseUrl + "metadata").then((response) => response.json());
  };

  componentDidMount() {
    this.getMetadata()
      .then((data) => {
        console.log(data);
        this.setState({ name: data.name });
      })
      .catch((error) => {
        console.log(error);
        this.setState({ name: "Error" });
      });
  }

  render() {
    return (
      <div className="App">
        <h1>
          <center> How is {this.state.name} ?</center>
        </h1>
        <Divider orientation="left" className={styles.divider}>
          {" "}
          Mental Health{" "}
        </Divider>
        <Row gutter={[16, 16]}>
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={5}
            name="energyLevels"
            displayName={"Energy"}
            url={this.dataUrl}
          />
           <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={5}
            name="happyLevels"
            displayName={"Happiness"}
            url={this.dataUrl}
          />
          <CalendarViz
            isPositive={false}
            isReverse={true}
            minRange={0}
            maxRange={5}
            name="anxietyLevels"
            displayName={"Anxiety"}
            url={this.dataUrl}
          />
         <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={5}
            name="excitedAboutFuture"
            displayName={"Excitement about Future"}
            url={this.dataUrl}
          />
        </Row>
        <br />
        <br />
        <Divider orientation="left" className={styles.divider}>
          {" "}
          Health{" "}
        </Divider>
        <Row gutter={[16, 16]}>
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="gym"
            displayName={"Workout"}
            url={this.dataUrl}
          />
           <CalendarViz
            isPositive={false}
            isReverse={true}
            minRange={0}
            maxRange={5}
            name="alcoholIntake"
            displayName={"Alcohol Intake"}
            url={this.dataUrl}
          />
         <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={5}
            name="numberOfMeals"
            displayName={"Number of Meals"}
            url={this.dataUrl}
          />
         <CalendarViz
            isPositive={false}
            isReverse={true}
            minRange={0}
            maxRange={5}
            name="coke"
            displayName={"Aerated Drinks Intake"}
            url={this.dataUrl}
          />
          <CalendarViz
            isPositive={false}
            isReverse={true}
            minRange={0}
            maxRange={5}
            name="lays"
            displayName={"Junk Food Intake"}
            url={this.dataUrl}
          />
           <CalendarViz
            isPositive={false}
            isReverse={true}
            minRange={0}
            maxRange={5}
            name="caffeine"
            displayName={"Caffeine Intake"}
            url={this.dataUrl}
          />
          <CalendarViz
            isPositive={false}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="didVomit"
            displayName={"Vomit in Morning"}
            url={this.dataUrl}
          />
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="sleepWokeUpYourself"
            displayName={"Woke up Yourself"}
            url={this.dataUrl}
          />
        </Row>
        <br />
        <br />
        <Divider orientation="left" className={styles.divider}>
          {" "}
          Productivity{" "}
        </Divider>
        <Row gutter={[16, 16]}>
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={5}
            name="learned"
            displayName={"Learning"}
            url={this.dataUrl}
          />
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="didRead"
            displayName={"Read"}
            url={this.dataUrl}
          />
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="goalsNextDay"
            displayName={"Planned Next Day"}
            url={this.dataUrl}
          />
          <CalendarViz
            isPositive={false}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="watchedTV"
            displayName={"Did watch TV"}
            url={this.dataUrl}
          />
        </Row>
        <br />
        <br />
        <Divider orientation="left" className={styles.divider}>
          {" "}
          Social{" "}
        </Divider>
        <Row gutter={[16, 16]}>
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="parents"
            displayName={"Talked to Family"}
            url={this.dataUrl}
          />
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="socializing"
            displayName={"Socialized"}
            url={this.dataUrl}
          />
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="out"
            displayName={"Went Out"}
            url={this.dataUrl}
          />
        </Row>
        <br />
        <br />
        <Divider orientation="left" className={styles.divider}>
          {" "}
          Hobbies{" "}
        </Divider>
        <Row gutter={[16, 16]}>
          <CalendarViz
            isPositive={true}
            isReverse={false}
            minRange={0}
            maxRange={1}
            name="chess"
            displayName={"Played Chess"}
            url={this.dataUrl}
          />
        </Row>
        <br />
        <br />
      </div>
    );
  }
}

export default App;
