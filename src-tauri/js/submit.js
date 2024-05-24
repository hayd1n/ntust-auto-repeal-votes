// this would be set from rust side
// const DRY_RUN = true;

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

let xpath = `//form[@action="/JudgeCourseServ/JudgeQuestion/EditJudge"]//button[@type="submit"]`;
// let xpath = "";
if (DRY_RUN) xpath = `/html/body/div[2]/div/div/div[1]/div/a`;

let submitButton = getElementsByXPath(xpath);

if (submitButton.length > 0) {
  submitButton[0].click();
}
