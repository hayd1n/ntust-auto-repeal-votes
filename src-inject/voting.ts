import { getElementsByXPath } from "./common";

export function isVoting() {
  return (
    getElementsByXPath(`//a[text()='回科目列表' or text()='Back to list']`)
      .length > 0
  );
}

export function vote() {
  const keywords = ["可", "沒意見", "普通", "從不缺課", "超過4小時", "否"];

  const contains = keywords
    .map(
      (v, i) =>
        `contains(@value, '${v}')` + (i !== keywords.length - 1 ? " or" : "")
    )
    .join(" ");

  let radios = getElementsByXPath(
    `//input[@type='radio' and (${contains})]`
  ) as HTMLInputElement[];

  // console.log(radios);

  radios.forEach((el) => (el.checked = true));

  let textareas = getElementsByXPath(
    `//form[@action="/JudgeCourseServ/JudgeQuestion/EditJudge"]//fieldset//textarea`
  ) as HTMLTextAreaElement[];

  textareas.forEach((el) => (el.value = "無"));

  // console.log(textareas);

  window.scrollTo(0, document.body.scrollHeight);
}
