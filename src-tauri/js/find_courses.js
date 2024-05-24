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

let courses = getElementsByXPath(
  `//table[@id="DataTables_Table_0"]//tbody//tr[./td//a]`
);

//table[@id="DataTables_Table_0"]//tbody//tr[./td//a]//td[1]

const onclickPattern = /onclick="javascript:post\(([^)]+)\)"/;

let coursesDetails = courses.map((c) => {
  let courseNo = getElementsByXPath("./td[1]", c)[0].innerText;
  let courseName = getElementsByXPath("./td[2]", c)[0].innerText;
  let html = getElementsByXPath("./td[9]", c)[0].innerHTML;

  let payload = html.match(onclickPattern)[1].split(",").slice(1).join();

  return { courseNo, courseName, payload };
});

let courses_json = JSON.stringify(coursesDetails);

console.log(courses_json);

let encoded = encodeURI(courses_json);

let currentUrl = window.location.toString();

window.location = `${currentUrl}#courses${encoded}`;
