import { generate_chart } from "./chart_factory.js"


generate_chart(document.getElementById("chart-failprob"),
    "Failure probability", "failureProbability", "rgb(255, 99, 132)", "rgb(255, 99, 132, 0.5)", true)

generate_chart(document.getElementById("chart-fails"),
    "Total failures", "failures", "rgb(255, 99, 132)", "rgb(255, 99, 132, 0.5)", true)

generate_chart(document.getElementById("chart-meanqueue"),
    "Mean queue Length", "meanQueueLength", "rgb(255, 159, 64)", "rgb(255, 159, 64, 0.5)", true)

generate_chart(document.getElementById("chart-waittime"),
    "Wait time", "waitTime", "rgb(255, 159, 64)", "rgb(255, 159, 64, 0.5)", true)

generate_chart(document.getElementById("chart-quantity"),
    "Quantity", "quantity", "rgb(75, 192, 192)", "rgb(75, 192, 192, 0.5)", false)