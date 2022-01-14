import * as wasm from "i-know-your-pattern";

const BLACK = 0;
const WHITE = 1;

const body = document.getElementsByTagName("body")[0];
const inputsDisplay = document.getElementById("inputs-display");
const predictionsDisplay = document.getElementById("predictions-display");
const correctDisplay = document.getElementById("correct-display");
const ratioDisplay = document.getElementById("ratio-display");
const nextPredictionDisplay = document.getElementById("next-prediction-display");

let inputCounter = 0;
let predictionCounter = 0;
let correctPredictions = 0;

function updateStatisticsTable() {
    inputsDisplay.innerHTML = inputCounter;
    predictionsDisplay.innerHTML = predictionCounter;
    correctDisplay.innerHTML = correctPredictions;

    if (predictionCounter > 0) {
        ratioDisplay.innerHTML = (correctPredictions / predictionCounter).toFixed(2);
    }

    const nextPrediction = wasm.predict_next_input();
    if (nextPrediction == WHITE) {
        nextPredictionDisplay.className = "light";
    }
    if (nextPrediction == BLACK) {
        nextPredictionDisplay.className = "dark";
    }
}

function pushInput(input) {
    inputCounter += 1;

    const prediction = wasm.predict_next_input();

    if (prediction != undefined) {
        predictionCounter += 1;

        if (prediction == input) {
            correctPredictions += 1;
        }
    }

    if (input == WHITE) {
        wasm.push_white_input();
        body.className = "light";
    } else {
        wasm.push_black_input();
        body.className = "dark";
    }

    updateStatisticsTable();
}

document.getElementById("black-input-button").onmousedown = () => {
    pushInput(BLACK);
};

document.getElementById("white-input-button").onmousedown = () => {
    pushInput(WHITE);
};
