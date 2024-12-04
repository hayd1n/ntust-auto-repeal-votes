export function getElementsByXPath(xpath: string, parent?: Node | null) {
  let results: (Node | null)[] = [];
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

export function getBase64Image(img: HTMLImageElement) {
  // Create an empty canvas element
  const canvas = document.createElement("canvas");
  canvas.width = img.width;
  canvas.height = img.height;

  // Copy the image contents to the canvas
  const ctx = canvas.getContext("2d");
  ctx?.drawImage(img, 0, 0);

  // Get the data-URL formatted image
  // Firefox supports PNG and JPEG. You could check img.src to
  // guess the original format, but be aware the using "image/jpg"
  // will re-encode the image.
  const dataURL = canvas.toDataURL("image/png");

  return dataURL;
}

export function isLogined() {
  return (
    getElementsByXPath(
      `//a[contains(@class, 'navbar-brand') and (text()='課程評量系統' or text()='Course Teaching Evaluation System')]`
    ).length > 0
  );
}
