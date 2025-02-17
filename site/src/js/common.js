export function setFeedback(success, message) {
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

export function getForm(sm, sm_no, lg) {
    let form = new FormData(document.forms['matchsticksValidate']);
    let fnToCall = null;
    let calcName = null;
    if (String(form.get('answer')) === "" || Number(form.get('matchsticksInput')) === 0) {
        setFeedback(false, "Matchsticks and Digits string must be filled in!")
    } else if (document.getElementById('smallestRadio').checked) {
        fnToCall = sm;
        calcName = 'smallest value';
    } else if (document.getElementById('smallestNoZeroRadio').checked) {
        fnToCall = sm_no;
        calcName = 'smallest value with no leading zeroes';
    } else if (document.getElementById('largestRadio').checked) {
        fnToCall = lg;
        calcName = 'largest value';
    } else {
        setFeedback(false, "Calculation type not selected!")
    }

    if (fnToCall === null) {
        return null;
    } else {
        return {
            form: form,
            fnToCall: fnToCall,
            calcName: calcName
        }
    }
}