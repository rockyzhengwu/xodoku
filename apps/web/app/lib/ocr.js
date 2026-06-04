export function ocrPostProcess(imgw, imgh, boxes) {
  const cellw = imgw / 9;
  const cellh = imgh / 9;
  const res = {};
  for (let i = 0; i < 81; i++) {
    res[i] = { pms: new Set(), digit: "0" };
  }
  for (let i = 0; i < boxes.length; i++) {
    const box = boxes[i];
    const label = box[4];
    const x1 = box[0];
    const y1 = box[1];
    const x2 = box[2];
    const y2 = box[3];
    const isnote = isNote(cellw, cellh, x1, y1, x2, y2);
    const index = cellIndex(cellw, cellh, x1, y1, x2, y2);
    if (isnote) {
      res[index].pms.add(label);
    } else {
      res[index].digit = label;
    }
  }
  return res;
}
function cellIndex(cellw, cellh, x1, y1, x2, y2) {
  const cx = x1 + (x2 - x1) / 2;
  const cy = y1 + (y2 - y1) / 2;
  const row = Math.floor(cy / cellh);
  const col = Math.floor(cx / cellw);
  const n = row * 9 + col;
  return n;
}
function isNote(cellw, cellh, x1, y1, x2, y2) {
  const w = Math.abs(x2 - x1);
  const h = Math.abs(y2 - y1);
  if (w < cellw * 0.5 && h < cellh * 0.5) {
    return true;
  }
  return false;
}
