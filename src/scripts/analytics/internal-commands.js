setTimeout(async () => {
  const blockLimit = 500;
  const groupSize = 10;

  let chartDom = document.getElementById("chart");
  window.addEventListener("resize", function () {
    myChart.resize();
  });
  let myChart = echarts.init(chartDom);

  myChart.showLoading({
    text: "Loading...", // Display text with the spinner
    color: "#E39844", // Spinner color
    zlevel: 0,
  });

  let response = await fetch(config.graphql_endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: `query MyQuery() {
        feetransfers(query: { canonical: true }, sortBy: BLOCKHEIGHT_DESC, limit: ${blockLimit}) {
          fee,
          blockStateHash {
            protocolState {
              consensusState {
                slotSinceGenesis
              }
            }
          }
        }
      } 
    `,
    }),
  });

  let jsonResp = await response.json();
  let data = jsonResp.data.feetransfers.reduce((agg, record) => {
    let slot =
      record.blockStateHash.protocolState.consensusState.slotSinceGenesis;
    let key = slot - (slot % groupSize);
    let value = record.fee;
    if (!agg[key]) {
      agg[key] = [];
    }
    let parsedFloat = parseFloat(value / 1e9);
    if (parsedFloat < 700) {
      agg[key].push(parsedFloat);
    }
    return agg;
  }, {});

  let xAxis = Object.keys(data);
  let option;

  myChart.hideLoading();

  option = {
    title: {
      text: `Fee Transfers in the last ${blockLimit} blocks`,
      left: "center",
    },
    tooltip: {
      trigger: "item",
      axisPointer: {
        type: "shadow",
      },
    },
    dataset: [
      {
        source: Object.entries(data).map(([_, fees]) => [...fees]),
      },
      {
        fromDatasetIndex: 0,
        transform: {
          type: "boxplot",
        },
      },
      {
        fromDatasetIndex: 1,
        fromTransformResult: 1,
      },
    ],
    xAxis: {
      type: "category",
      name: "Global Slot",
      axisLabel: {
        formatter: function (value) {
          return xAxis[value];
        },
      },
    },
    yAxis: [
      {
        type: "value",
        name: "Fee",
        axisLabel: {
          formatter: function (value) {
            return `${value} MINA`;
          },
        },
      },
      {
        type: "value",
        name: "Transfers Count",
        position: "right",
        axisLabel: {
          formatter: function (value) {
            return `${value}`;
          },
        },
      },
    ],
    series: [
      {
        name: "boxplot",
        type: "boxplot",
        datasetIndex: 1,
        yAxisIndex: 0,
        tooltip: {
          formatter: function (param) {
            return [
              "Slot: " +
                xAxis[param.name] +
                `-${+xAxis[param.name] + groupSize - 1}`,
              "upper: " + param.data[5],
              "Q3: " + param.data[4],
              "median: " + param.data[3],
              "Q1: " + param.data[2],
              "lower: " + param.data[1],
            ].join("<br/>");
          },
        },
      },
      {
        name: "boxplot",
        type: "scatter",
        symbolSize: 8,
        datasetIndex: 2,
        yAxisIndex: 0,
        tooltip: {
          formatter: function (param) {
            return [
              "Slot: " +
                xAxis[param.name] +
                `-${+xAxis[param.name] + groupSize - 1}`,
              "Fee: " + param.data[1],
            ].join("<br/>");
          },
        },
      },
      {
        name: "transfers",
        type: "scatter",
        symbolSize: 12,
        data: Object.values(data).map((fees, i) => ["" + i, fees.length]),
        yAxisIndex: 1,
        tooltip: {
          formatter: function (param) {
            return `Slot: ${xAxis[param.dataIndex]}-${+xAxis[param.dataIndex] + groupSize - 1}<br/>Transfers: ${param.value[1]}`;
          },
        },
      },
    ],
  };

  option && myChart.setOption(option);
}, 1000);
