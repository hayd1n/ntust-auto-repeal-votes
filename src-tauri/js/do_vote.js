function getElementsByXPath(xpath, parent) {
  let results = [];
  let query = document.evaluate(
    xpath,
    parent || document,
    null,
    XPathResult.ORDERED_NODE_SNAPSHOT_TYPE,
    null
  );
  for (let i = 0, length = query.snapshotLength; i < length; ++i) {
    results.push(query.snapshotItem(i));
  }
  return results;
}

let keywords = ["可", "沒意見", "普通", "從不缺課", "超過4小時", "否"];

let contains = keywords
  .map(
    (v, i) =>
      `contains(@value, '${v}')` + (i !== keywords.length - 1 ? " or" : "")
  )
  .join(" ");

let radios = getElementsByXPath(`//input[@type='radio' and (${contains})]`);

// console.log(radios);

radios.forEach((el) => (el.checked = true));

let textareas = getElementsByXPath(
  `//form[@action="/JudgeCourseServ/JudgeQuestion/EditJudge"]//fieldset//textarea`
);

textareas.forEach((el) => (el.value = "無"));

// console.log(textareas);

window.scrollTo(0, document.body.scrollHeight);
