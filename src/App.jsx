import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Graph from './components/Graph';
import Port from './components/Port';


function App() {
  const [greetMsg, setGreetMsg] = useState([]); // Initial empty array for data
  const [name, setName] = useState("");
  const [startTime, setStartTime] = useState(1744); // Initial start time
  const [endTime, setEndTime] = useState(1755800); // Initial end time
  const [loading, setLoading] = useState(false); // Track loading state

  async function fetchTelemetryData() {
    try {
      setLoading(true); // Start loading
      // Fetch data with current startTime and endTime
      const response = await invoke("getdata1", { starttime: startTime, endtime: endTime });

      // Append new data to the existing data array
      setGreetMsg((prevData) => [...prevData, ...response]);
      
      // Adjust the startTime and endTime for next fetch
      const nextStartTime = endTime;
      const nextEndTime = endTime + 10; // Increment by certain number 
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
    <Port/>
      <h1 >Telemetry Data</h1>
    
      <div className='flex gap-[300px]'>

        <Graph data2DArray={greetMsg} fetchMoreData={fetchTelemetryData} loading={loading} var1="gyroX" var2="gyroY" var3="gyroZ" />
        <Graph data2DArray={greetMsg} fetchMoreData={fetchTelemetryData} loading={loading} var1="accelX" var2="accelY" var3="accelZ" />
      </div>
    </>
  );
}

export default App;
