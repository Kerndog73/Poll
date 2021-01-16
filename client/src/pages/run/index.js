const progress = document.getElementById("run-progress");
const source = new EventSource(EVENT_URL);

source.onmessage = e => {
    const pair = e.data.split(",");
    progress.innerText = `Submitted: ${pair[0]}, in-progress: ${pair[1]}`;
};
