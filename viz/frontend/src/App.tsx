import React, { useEffect, useState } from "react";
import "./App.scss";
import VizBuilder from "./components/VizBuilder";

function App() {
  const [name, setName] = useState("unnamed");
  const development: boolean =
    !process.env.NODE_ENV || process.env.NODE_ENV === "development";
  // const baseUrl: string = development
    // ? "http://msi.local:8080/api/"
    // : "/api/";
  const baseUrl: string = "https://metrics.charutak.xyz/api/"; 
  
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
      <VizBuilder baseUrl={baseUrl}></VizBuilder>
    </div>
  );
}

export default App;
