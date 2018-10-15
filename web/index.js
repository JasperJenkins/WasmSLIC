import slic from "../crate/Cargo.toml"

const image = document.querySelector("img");
const canvas = document.querySelector("canvas");
const context = canvas.getContext("2d");
const segments_input = document.querySelector("#segments-input");
const compactness_input = document.querySelector("#compactness-input");
const segments_label = document.querySelector("#segments-label");
const compactness_label = document.querySelector("#compactness-label");

segments_input.onchange = upadate_the_thing;

compactness_input.onchange = upadate_the_thing;

function upadate_the_thing() {
    context.canvas.width = image.width;
    context.canvas.height = image.height + 4;
    context.drawImage(image, 0, 0, image.width, image.height);
    let image_data = context.getImageData(0, 0, image.width, image.height);
    context.putImageData(image_data, 0, 0);
    segments_label.innerText = "Segments (8-512): " + segments_input.value;
    compactness_label.innerText = "Compactness 10^(1-2.5): " + compactness_input.value;
    image_data = slic.segment_image(
        +segments_input.value,
        Math.pow(10, +compactness_input.value),
        image_data
    );
    context.putImageData(image_data, 0, 0);
}

if (image.complete) {
    upadate_the_thing();
} else {
    image.onload = upadate_the_thing;
}