import React, { useEffect, useState } from "react";
import "./App.scss";
import VizBuilder from "./components/VizBuilder";
import DateElement from "./components/DateElements";

function App() {
  const [name, setName] = useState("unnamed");
  const baseUrl: string = process.env.REACT_APP_API_URL || "/api/";

  useEffect(() => {
    fetch(baseUrl + "metadata")
      .then((response) => response.json())
      .then((data) => {
        setName(data.name);
      })
      .catch((error) => {
        console.log(error);
        setName("error");
      });
  }, [baseUrl]);

  return (
    <div className="App">
      <div className="app-header">
        <h1>How is {name}? 🤔</h1>
      </div>
      <div className="dateHeading">
        <DateElement></DateElement>
      </div>
      <VizBuilder baseUrl={baseUrl}></VizBuilder>
    </div>
  );
}

export default App;
