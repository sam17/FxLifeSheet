import React, { useEffect, useState } from "react";
import "./App.scss";
import { Row, Col, Divider } from "antd";
import CalendarViz from "./components/CalendarViz";
import styles from "./stylesheets.module.scss";

function App() {
  const [name, setName] = useState("unnamed");
  const development: boolean =
    !process.env.NODE_ENV || process.env.NODE_ENV === "development";
  const baseUrl: string = development
    ? "https://metrics.soumyadeep.in/api/"
    : "/api/";
  const dataUrl = baseUrl + "data/";

  const getMetadata = () => {
    return fetch(baseUrl + "metadata").then((response) => response.json());
  };

  useEffect(() => {
    getMetadata()
      .then((data) => {
        console.log(data);
        setName(data.name);
      })
      .catch((error) => {
        console.log(error);
        setName("error");
      });
  }, []);

  return (
    <div className="App">
      <h1>
        <center> How is {name} ?</center>
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
          url={dataUrl}
        />
        <CalendarViz
          isPositive={true}
          isReverse={false}
          minRange={0}
          maxRange={5}
          name="happyLevels"
          displayName={"Happiness"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={false}
          isReverse={true}
          minRange={0}
          maxRange={5}
          name="anxietyLevels"
          displayName={"Anxiety"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={true}
          isReverse={false}
          minRange={0}
          maxRange={5}
          name="excitedAboutFuture"
          displayName={"Excitement about Future"}
          url={dataUrl}
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
          url={dataUrl}
        />
        <CalendarViz
          isPositive={false}
          isReverse={true}
          minRange={0}
          maxRange={5}
          name="alcoholIntake"
          displayName={"Alcohol Intake"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={true}
          isReverse={false}
          minRange={0}
          maxRange={5}
          name="numberOfMeals"
          displayName={"Number of Meals"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={false}
          isReverse={true}
          minRange={0}
          maxRange={5}
          name="coke"
          displayName={"Aerated Drinks Intake"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={false}
          isReverse={true}
          minRange={0}
          maxRange={5}
          name="lays"
          displayName={"Junk Food Intake"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={false}
          isReverse={true}
          minRange={0}
          maxRange={5}
          name="caffeine"
          displayName={"Caffeine Intake"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={false}
          isReverse={false}
          minRange={0}
          maxRange={1}
          name="didVomit"
          displayName={"Vomit in Morning"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={true}
          isReverse={false}
          minRange={0}
          maxRange={1}
          name="sleepWokeUpYourself"
          displayName={"Woke up Yourself"}
          url={dataUrl}
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
          url={dataUrl}
        />
        <CalendarViz
          isPositive={true}
          isReverse={false}
          minRange={0}
          maxRange={1}
          name="didRead"
          displayName={"Read"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={true}
          isReverse={false}
          minRange={0}
          maxRange={1}
          name="goalsNextDay"
          displayName={"Planned Next Day"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={false}
          isReverse={false}
          minRange={0}
          maxRange={1}
          name="watchedTV"
          displayName={"Did watch TV"}
          url={dataUrl}
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
          url={dataUrl}
        />
        <CalendarViz
          isPositive={true}
          isReverse={false}
          minRange={0}
          maxRange={1}
          name="socializing"
          displayName={"Socialized"}
          url={dataUrl}
        />
        <CalendarViz
          isPositive={true}
          isReverse={false}
          minRange={0}
          maxRange={1}
          name="out"
          displayName={"Went Out"}
          url={dataUrl}
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
          url={dataUrl}
        />
      </Row>
      <br />
      <br />
    </div>
  );
}

export default App;
