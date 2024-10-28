import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Graph from './components/Graph';
import Port from './components/Port';

function App() {
  const [greetMsg, setGreetMsg] = useState([]); // Initial empty array for data from getdata1
  const [greetMsg2, setGreetMsg2] = useState([]); // Initial empty array for data from getdata2
  const [startTime, setStartTime] = useState(1744); // Initial start time for getdata1
  const [endTime, setEndTime] = useState(1755800); // Initial end time for getdata1
  const [loading, setLoading] = useState(false); // Track loading state

  async function fetchTelemetryData() {
    try {
      setLoading(true); // Start loading
      // Fetch data with current startTime and endTime from getdata1
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

  async function fetchTelemetryData2() {
    try {
      // Fetch data from getdata2 (use the same time range for simplicity)
      const response = await invoke("getdata2", { starttime: startTime, endtime: endTime });
      // Append new data to the existing data array for getdata2
      setGreetMsg2((prevData) => [...prevData, ...response]);
    } catch (error) {
      console.error("Error fetching telemetry data from getdata2:", error);
    }
  }

  // Use useEffect to load the initial data on mount and set up an interval
  useEffect(() => {
    fetchTelemetryData(); // Initial fetch for getdata1
    fetchTelemetryData2(); // Initial fetch for getdata2

    const intervalId = setInterval(() => {
      fetchTelemetryData(); // Fetch data from getdata1
      fetchTelemetryData2(); // Fetch data from getdata2
    }, 10000); // Fetch every 10 seconds

    return () => clearInterval(intervalId); // Cleanup interval on unmount
  }, []);

  return (
    <>
      <Port />
      <h1>Telemetry Data</h1>
      <div className='flex gap-[300px]'>
        <Graph data2DArray={greetMsg} fetchMoreData={fetchTelemetryData} loading={loading} var1="gyroX" var2="gyroY" var3="gyroZ" />
        {/* <Graph data2DArray={greetMsg2} fetchMoreData={fetchTelemetryData2} loading={loading} var1="accelX" var2="accelY" var3="accelZ" /> */}
      </div>
      {/* <div>
        
        {greetMsg2}
      </div> */}
    </>
  );
}

export default App;
