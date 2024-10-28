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
  const [greetMsg2, setGreetMsg2] = useState([]); // Initial empty array for data from getdata2
  const [startTime2, setStartTime2] = useState(0); // Initial start time for getdata2
  const [endTime2, setEndTime2] = useState(10); // Initial end time for getdata2
  const [loading2, setLoading2] = useState(false); // Track loading state for getdata2

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

  async function fetchTelemetryData2() {
    try {
      setLoading2(true); // Start loading for getdata2
      // Fetch data with current startTime2 and endTime2
      const response = await invoke("getdata2", { starttime: startTime2, endtime: endTime2 });

      // Append new data to the existing data array for getdata2
      setGreetMsg2((prevData) => [...prevData, ...response]);
      
      // Adjust the startTime2 and endTime2 for next fetch
      const nextStartTime2 = endTime2;
      const nextEndTime2 = endTime2 + 10; // Increment by certain number 
      setStartTime2(nextStartTime2);
      setEndTime2(nextEndTime2);

      setLoading2(false); // Stop loading for getdata2
    } catch (error) {
      console.error("Error fetching telemetry data 2:", error);
      setLoading2(false);
    }
  }

  // Use useEffect to load the initial data on mount
  useEffect(() => {
    fetchTelemetryData(); // Initial fetch for getdata1
    fetchTelemetryData2(); // Initial fetch for getdata2
  }, []);

  return (
    <>
      <Port/>
      <h1>Telemetry Data</h1>
    
      <div className='flex gap-[300px]'>

        <Graph data2DArray={greetMsg} fetchMoreData={fetchTelemetryData} loading={loading} var1="gyroX" var2="gyroY" var3="gyroZ" />
        {/* <Graph data2DArray={greetMsg} fetchMoreData={fetchTelemetryData} loading={loading} var1="accelX" var2="accelY" var3="accelZ" /> */}
        
        <Graph data2DArray={greetMsg2} fetchMoreData={fetchTelemetryData2} loading={loading2} var1="otherVar1" var2="otherVar2" var3="otherVar3" />
        {/* <Graph data2DArray={greetMsg2} fetchMoreData={fetchTelemetryData2} loading={loading2} var1="otherVar4" var2="otherVar5" var3="otherVar6" /> */}
      </div>
    </>
  );
}

export default App;
