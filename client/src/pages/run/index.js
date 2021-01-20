const progress = document.getElementById("run-progress");
const count = document.getElementById("run-progress-count");
const expire = document.getElementById("run-expire-time");
const source = new EventSource(EVENT_URL);
const date = new Date(EXPIRE * 1000);

source.onmessage = e => {
    count.innerText = e.data;
    // https://css-tricks.com/restart-css-animation/#update-another-javascript-method-to-restart-a-css-animation
    progress.classList.remove("flash-animation");
    void progress.offsetWidth;
    progress.classList.add("flash-animation");
};

expire.datetime = date.toISOString();
expire.innerText = date.toLocaleString();

setTimeout(() => {
    expire.className = "text-danger";
}, date - new Date());
