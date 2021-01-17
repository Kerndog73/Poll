const minimum = document.getElementById("minimum");
const maximum = document.getElementById("maximum");
const integer = document.getElementById("integer");
const negateMinimum = document.getElementById("negate-minimum");
const negateMaximum = document.getElementById("negate-maximum");

function parse(element, def) {
    if (element.value === "") {
        element.setCustomValidity("");
        return def;
    }
    return parseFloat(element.value);
}

function validate() {
    const min = parse(minimum, Number.NEGATIVE_INFINITY);
    const max = parse(maximum, Number.POSITIVE_INFINITY);
    if (isNaN(min) || isNaN(max)) return;
    minimum.max = max;
    maximum.min = min;
    if (min >= max) {
        minimum.setCustomValidity("Must be smaller than maximum");
        maximum.setCustomValidity("Must be greater than minimum");
    } else {
        minimum.setCustomValidity("");
        maximum.setCustomValidity("");
    }
}

function adjustStep() {
    if (integer.checked) {
        minimum.step = "1";
        maximum.step = "1";
        minimum.inputMode = "numeric";
        maximum.inputMode = "numeric";
    } else {
        minimum.step = "any";
        maximum.step = "any";
        minimum.inputMode = "decimal";
        maximum.inputMode = "decimal";
    }
}

minimum.addEventListener("input", validate);
maximum.addEventListener("input", validate);
integer.addEventListener("input", validate);
integer.addEventListener("input", adjustStep);

negateMinimum.addEventListener("click", () => {
    minimum.value = -minimum.value;
    validate();
});

negateMaximum.addEventListener("click", () => {
    maximum.value = -maximum.value;
    validate();
});
