import '../scss/styles.scss';

import {
    check_smallest, check_smallest_no_leading_zeroes, check_largest
} from "matchsticks-wasm";
import {getForm, setFeedback} from "./common";

document.forms['matchsticksValidate'].onsubmit = function(e) {
    let formInfo = getForm(check_smallest, check_smallest_no_leading_zeroes, check_largest);

    if (formInfo !== null) {
        let { form, fnToCall, calcName } = formInfo;
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

