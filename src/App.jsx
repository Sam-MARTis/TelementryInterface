import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Graph from './components/Graph';
import Port from './components/Port';

function App() {
  const [greetMsg, setGreetMsg] = useState([]); // Data from getdata1
  const [data2, setData2] = useState([]); // Data from getdata2
  const [startTime, setStartTime] = useState(1744); // Initial start time for getdata1
  const [endTime, setEndTime] = useState(1755800); // Initial end time for getdata1
  const [loading, setLoading] = useState(false); // Track loading state

  async function fetchTelemetryData() {
    try {
      setLoading(true); // Start loading
      const response = await invoke("getdata1", { starttime: startTime, endtime: endTime });
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

  // Fetch data from getdata2 every second
  useEffect(() => {
    const fetchData2 = async () => {
      try {
        const response = await invoke("getdata2", { starttime: 1749743, endtime: 1918417 });
        setData2(response);
      } catch (error) {
        console.error("Error fetching data2:", error);
      }
    };

    const intervalId = setInterval(fetchData2, 1000); // Fetch data every second

    return () => clearInterval(intervalId); // Cleanup on unmount
  }, []);

  // Use useEffect to load the initial data on mount
  useEffect(() => {
    fetchTelemetryData(); // Initial fetch
  }, []);

  return (
    <>
      <Port/>
      <h1>Telemetry Data</h1>
      
      <div className='flex gap-[300px]'>
        <Graph data2DArray={greetMsg} fetchMoreData={fetchTelemetryData} loading={loading} var1="gyroX" var2="gyroY" var3="gyroZ" />
        {/* <Graph data2DArray={greetMsg} fetchMoreData={fetchTelemetryData} loading={loading} var1="accelX" var2="accelY" var3="accelZ" /> */}
      </div>

      {/* Display data2 in a formatted way */}
      <div className="data-display">
        <h2>Data 2</h2>
        <table className="table-auto border-collapse border border-gray-200">
          <thead>
            <tr>
              <th className="border border-gray-300 px-4 py-2">Column 1</th>
              <th className="border border-gray-300 px-4 py-2">Column 2</th>
              <th className="border border-gray-300 px-4 py-2">Column 3</th>
              {/* Add more headers as needed */}
            </tr>
          </thead>
          <tbody>
            {data2.map((row, index) => (
              <tr key={index}>
                <td className="border border-gray-300 px-4 py-2">{row[1]}</td>
                <td className="border border-gray-300 px-4 py-2">{row[11]}</td>
                <td className="border border-gray-300 px-4 py-2">{row[12]}</td>
                {/* Add more columns based on the structure of data2 */}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </>
  );
}

export default App;
