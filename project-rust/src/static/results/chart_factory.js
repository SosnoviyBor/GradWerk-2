export function generate_chart(element, label, param, border_color, bg_color, only_process) {
    return new Chart(element, {
        type: 'bar',
        data: {
            labels: parseData("name", only_process).map(row => row.name),
            datasets: [
                {
                    label: label,
                    labelSize: 20,
                    data: parseData(param, only_process).map(row => row.value),
                    borderColor: border_color,
                    borderWidth: 2,
                    backgroundColor: bg_color,
                },
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                title: {
                    display: true,
                    text: label,
                    font: {
                        size: 16
                    }
                }
            }
        }
    });
}


function parseData(value, only_process) {
    const data = Array.from(document.getElementsByClassName("element"))
        .filter(element =>
            !(only_process && element.dataset.elementType.toLowerCase() !== "process")
        )
        .map(element => ({
            name: element.dataset.name,
            value: element.dataset[value]
        }))
        .sort((a, b) => a.name.localeCompare(b.name));

    return data;
}
