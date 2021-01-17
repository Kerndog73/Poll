const negate = document.getElementById("negate");
const box = document.getElementById("box");
const slider = document.getElementById("slider");

if (slider) {
    slider.oninput = () => {
        box.value = slider.value;
    };
    box.oninput = () => {
        slider.value = box.value;
    };
}

if (negate) {
    negate.onclick = () => {
        box.value = -box.value;
        if (slider) {
            slider.value = -slider.value;
        }
    };
}
