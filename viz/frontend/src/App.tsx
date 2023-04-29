import React, { useEffect, useState } from "react";
import "./App.scss";
import VizBuilder from "./components/VizBuilder";
import DateElement from "./components/DateElements";

function App() {
  const [name, setName] = useState("unnamed");
  const development: boolean =
    !process.env.NODE_ENV || process.env.NODE_ENV === "development";
  const baseUrl: string = development
    ? // ? "http://localhost:8080/api/"
      "https://metrics.soumyadeep.in/api/"
    : "/api/";

  const dataUrl = baseUrl + "data/";
  const getMetadata = () => {
    return fetch(baseUrl + "metadata").then((response) => response.json());
  };

  useEffect(() => {
    getMetadata()
      .then((data) => {
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
        <center> How is {name}? 🤔</center>
      </h1>
      <div className="dateHeading">
        <center>
          <DateElement></DateElement>
        </center>
      </div>
      <VizBuilder baseUrl={baseUrl}></VizBuilder>
    </div>
  );
}

export default App;
