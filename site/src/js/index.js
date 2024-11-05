import '../scss/styles.scss';

import { calculate_smallest_no_leading_zeroes, calculate_smallest, calculate_largest } from "matchsticks-wasm";

function setFeedback(success, message) {
    let el = document.getElementById('feedbackMessage');
    el.textContent = message;
    if (success) {
        el.classList.remove('text-danger');
        el.classList.add('text-success');
    } else {
        el.classList.remove('text-success');
        el.classList.add('text-danger');
    }
}

document.forms['matchsticksValidate'].onsubmit = function(e) {
    let form = new FormData(document.forms['matchsticksValidate']);
    let fnToCall = null;
    let calcName = null;
    if (String(form.get('answer')) === "" || Number(form.get('matchsticksInput')) === 0) {
        setFeedback(false, "Matchsticks and Result String must be filled in!")
    } else if (document.getElementById('smallestRadio').checked) {
        fnToCall = calculate_smallest;
        calcName = 'smallest value';
    } else if (document.getElementById('smallestNoZeroRadio').checked) {
        fnToCall = calculate_smallest_no_leading_zeroes;
        calcName = 'smallest value with no leading zeroes';
    } else if (document.getElementById('largestRadio').checked) {
        fnToCall = calculate_largest;
        calcName = 'largest value';
    } else {
        setFeedback(false, "Calculation type not selected!")
    }

    if (fnToCall !== null && fnToCall !== undefined) {
        let matchsticks = Number(form.get('matchsticksInput'));
        let ans = String(form.get('answer'));
        let result = fnToCall(matchsticks, ans);
        console.log(result)
        if (result.error) {
            setFeedback(false, result.error)
        } else if (result.correct) {
            setFeedback(true, `The ${calcName} for ${matchsticks} matchsticks is ${ans}!`)
        } else {
            setFeedback(false, `The ${calcName} for ${matchsticks} matchsticks is NOT ${ans}!`)
        }
    }

    e.preventDefault();
    return false;
}

