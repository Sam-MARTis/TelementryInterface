import React, { useState, useEffect } from 'react';
import './Port.css';

const Port = () => {
  const [ports, setPorts] = useState([]);
  const [selectedPort, setSelectedPort] = useState('');
  const [theme, setTheme] = useState('blue'); // Default theme

  useEffect(() => {
    const fetchPorts = async () => {
      try {
        const response = await fetch('/api/available-ports');
        const data = await response.json();
        setPorts(data.ports);
      } catch (error) {
        console.error('Error fetching ports:', error);
      }
    };

    fetchPorts();
  }, []);

  const handlePortChange = async (event) => {
    const chosenPort = event.target.value;
    setSelectedPort(chosenPort);

    try {
      await fetch(`/api/set-port`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ port: chosenPort }),
      });
      console.log(`Port set to ${chosenPort}`);
    } catch (error) {
      console.error('Error setting port:', error);
    }
  };

  const handleThemeChange = (event) => {
    setTheme(event.target.value);
  };

  return (
    <div className={`port-container ${theme}`}>
      {/* <h2>Select Port</h2> */}
      <select className="port-dropdown" value={selectedPort} onChange={handlePortChange}>
        <option value="" disabled>
          Choose a port
        </option>
        {ports.map((port) => (
          <option key={port} value={port}>
            {port}
          </option>
        ))}
      </select>

      <select className="theme-selector" value={theme} onChange={handleThemeChange}>
        <option value="blue">Blue</option>
        <option value="red">Red</option>
        <option value="green">Green</option>
        <option value="purple">Purple</option>
        <option value="yellow">Yellow</option>
        <option value="teal">Teal</option>
        <option value="orange">Orange</option>
        <option value="pink">Pink</option>
      </select>
    </div>
  );
};

export default Port;
