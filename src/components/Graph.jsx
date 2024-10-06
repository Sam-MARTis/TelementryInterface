import React, { useEffect, useState } from 'react';
import { Line } from 'react-chartjs-2';
import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend } from 'chart.js';

// Register Chart.js components
ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend);

const Graph = ({ data2DArray, var1, var2, var3, fetchMoreData, loading }) => {
    const datasetConfig = [
        { label: var1, color: 'rgba(75,192,192,1)' },
        { label: var2, color: 'rgba(153, 102, 255, 1)' },
        { label: var3, color: 'rgba(255, 99, 132, 1)' },
    ];

    const [chartData, setChartData] = useState({
        labels: [],
        datasets: datasetConfig.map(sensor => ({
            label: sensor.label,
            data: [],
            borderColor: sensor.color,
            borderWidth: 2,
            fill: false,
            pointRadius: 0,
            pointHoverRadius: 0,
        })),
    });

    const [time, setTime] = useState(0);
    const [index, setIndex] = useState(0); // To track the current data index

    useEffect(() => {
        const interval = setInterval(() => {
            if (index < data2DArray.length) {
                const currentData = data2DArray[index];
                const value1 = currentData[12];
                const value2 = currentData[13];
                const value3 = currentData[14];

                setChartData((prevData) => {
                    const updatedLabels = [...prevData.labels, time];

                    const updatedDatasets = prevData.datasets.map((dataset, i) => ({
                        ...dataset,
                        data: [...dataset.data, i === 0 ? value1 : i === 1 ? value2 : value3],
                    }));

                    return {
                        labels: updatedLabels,
                        datasets: updatedDatasets,
                    };
                });

                setTime((prevTime) => prevTime + 1);
                setIndex((prevIndex) => prevIndex + 1);

                // If about to run out of data, trigger a fetch for more
                if (index >= data2DArray.length - 5 && !loading) {
                    fetchMoreData();
                }
            }
        }, 100);

        return () => clearInterval(interval);
    }, [index, time, data2DArray, fetchMoreData, loading]);

    return (
        <div>
            <h2>Multiple Sensor Data</h2>
            <Line
                data={chartData}
                options={{
                    responsive: true,
                    plugins: {
                        legend: { display: true },
                        title: {
                            display: true,
                            text: 'Sensor Data',
                        },
                    },
                    scales: {
                        x: {
                            type: 'linear',
                            title: { display: true, text: 'Time' },
                            min: time - 100,
                            max: time,
                        },
                        y: {
                            title: { display: true, text: 'Magnitude' },
                            min: -300,
                            max: 300,
                        },
                    },
                }}
            />
        </div>
    );
};

export default Graph;
