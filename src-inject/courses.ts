import { getElementsByXPath } from "./common";

export async function getCourses() {
  let courses = getElementsByXPath(
    `//table[@id="DataTables_Table_0"]//tbody//tr[./td//a]`
  );

  //table[@id="DataTables_Table_0"]//tbody//tr[./td//a]//td[1]

  const onclickPattern = /onclick="javascript:post\(([^)]+)\)"/;

  let coursesDetails = courses.map((c) => {
    const courseNoEl = getElementsByXPath(
      "./td[1]",
      c
    )[0] as HTMLTableCellElement;
    const courseNameEl = getElementsByXPath(
      "./td[2]",
      c
    )[0] as HTMLTableCellElement;
    const htmlEl = getElementsByXPath("./td[9]", c)[0] as HTMLTableCellElement;

    let courseNo = courseNoEl.innerText;
    let courseName = courseNameEl.innerText;
    let html = htmlEl.innerHTML;

    let match = html.match(onclickPattern);
    let payload = match ? match[1].split(",").slice(1).join() : "";

    return { courseNo, courseName, payload };
  });

  return coursesDetails;
}
