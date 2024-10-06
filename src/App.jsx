import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Graph from './components/Graph';

function App() {
  const [greetMsg, setGreetMsg] = useState([]); // Initial empty array for data
  const [name, setName] = useState("");
  const [startTime, setStartTime] = useState(1744); // Initial start time
  const [endTime, setEndTime] = useState(1755800); // Initial end time
  const [loading, setLoading] = useState(false); // Track loading state

  const [showGraph1, setShowGraph1] = useState(true);
  const [showGraph2, setShowGraph2] = useState(true);

  const toggleGraph1 = () => setShowGraph1(!showGraph1);
  const toggleGraph2 = () => setShowGraph2(!showGraph2);

  async function fetchTelemetryData() {
    try {
      setLoading(true); // Start loading
      // Fetch data with current startTime and endTime
      const response = await invoke("getdata1", { starttime: startTime, endtime: endTime });

      // Append new data to the existing data array
      setGreetMsg((prevData) => [...prevData, ...response]);
      
      // Adjust the startTime and endTime for next fetch
      const nextStartTime = endTime;
      const nextEndTime = endTime + 100; // Increment by 100
      setStartTime(nextStartTime);
      setEndTime(nextEndTime);

      setLoading(false); // Stop loading
    } catch (error) {
      console.error("Error fetching telemetry data:", error);
      setLoading(false);
    }
  }

  // Use useEffect to load the initial data on mount
  useEffect(() => {
    fetchTelemetryData(); // Initial fetch
  }, []);

  return (
    <>
      <h1>Telemetry Data</h1>
      <div className="button-section">
        <button onClick={toggleGraph1}>Toggle Graph 1</button>
        <button onClick={toggleGraph2}>Toggle Graph 2</button>
      </div>
      <div className='flex gap-[300px]'>

        {showGraph1 && <Graph data2DArray={greetMsg} fetchMoreData={fetchTelemetryData} loading={loading} var1="gyroX" var2="gyroY" var3="gyroZ" />}
        {showGraph2 && <Graph data2DArray={greetMsg} fetchMoreData={fetchTelemetryData} loading={loading} var1="accelX" var2="accelY" var3="accelZ" />}
      </div>
    </>
  );
}

export default App;
