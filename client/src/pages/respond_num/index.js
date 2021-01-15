const negate = document.getElementById("negate");
const box = document.getElementById("box");
const slider = document.getElementById("slider");

if (slider) {
    slider.addEventListener("input", () => {
        box.value = slider.value;
    });
    box.addEventListener("input", () => {
        slider.value = box.value;
    });
}

if (negate) {
    negate.addEventListener("click", () => {
        box.value = -box.value;
        if (slider) {
            slider.value = -slider.value;
        }
    });
}
