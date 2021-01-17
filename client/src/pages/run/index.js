const progress = document.getElementById("run-progress-count");
const source = new EventSource(EVENT_URL);

source.onmessage = e => {
    progress.innerText = e.data;
};
