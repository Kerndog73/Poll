const progress = document.getElementsByClassName("run-progress")[0];
const count = document.getElementById("run-progress-count");
const source = new EventSource(EVENT_URL);

source.onmessage = e => {
    count.innerText = e.data;
    // https://css-tricks.com/restart-css-animation/#update-another-javascript-method-to-restart-a-css-animation
    progress.classList.remove("flash-animation");
    void progress.offsetWidth;
    progress.classList.add("flash-animation");
};
