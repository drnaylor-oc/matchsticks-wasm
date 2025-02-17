import '../scss/styles.scss';

import { calculate_smallest, calculate_smallest_no_leading_zeroes, calculate_largest } from "matchsticks-wasm";
import {getForm, setFeedback} from "./common";

document.forms['matchsticksValidate'].onsubmit = function(e) {

    let formInfo = getForm(calculate_smallest, calculate_smallest_no_leading_zeroes, calculate_largest);

    if (formInfo !== null) {
        let { form, fnToCall } = formInfo;
        let matchsticks = Number(form.get('matchsticksInput'));
        let result = fnToCall(matchsticks);
        console.log(result)
        if (result.error) {
            setFeedback(false, result.error)
        } else {
            setFeedback(true, `Digits: ${result.length}\r\nNumber: ${result.result}`)
        }
    }

    e.preventDefault();
    return false;
}

