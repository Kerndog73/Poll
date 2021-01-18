const options = document.getElementById("options");
const firstOption = options.firstElementChild.firstElementChild;

const MAX_OPTIONS = 16;

function template(i) {
    return `<input class="form-control" type="text" maxlength="64" placeholder="Choice ${i}" aria-label="Choice ${i}"/>`;
}

function optionInput(e) {
    if (
        e.target.value.length &&
        e.target.parentElement === options.lastElementChild &&
        options.childElementCount < MAX_OPTIONS
    ) {
        const div = document.createElement("div");
        div.className = "form-group";
        div.innerHTML = template(options.childElementCount + 1);
        const input = div.firstElementChild;
        input.oninput = optionInput;
        input.onblur = optionBlur;
        input.onfocus = optionFocus;
        if (options.childElementCount === 1) {
            input.setAttribute("required", "");
        }
        options.appendChild(div);
    }
}

function optionBlur(e) {
    if (e.target.value.length) {
        e.target.setAttribute("name", "option");
    } else {
        e.target.removeAttribute("name");
        if (e.target.parentElement !== options.lastElementChild) {
            e.target.parentElement.remove();
            const children = options.children;
            const count = children.length;
            for (let i = 0; i !== count; ++i) {
                const input = children[i].firstElementChild;
                const text = "Choice " + (i + 1);
                input.setAttribute("placeholder", text);
                input.setAttribute("aria-label", text);
                if (i < 2) {
                    input.setAttribute("required", "");
                }
            }
        }
    }
}

function optionFocus(e) {
    e.target.parentElement.scrollIntoView({block: "nearest"});
}

firstOption.oninput = optionInput;
firstOption.onblur = optionBlur;
firstOption.onfocus = optionFocus;

